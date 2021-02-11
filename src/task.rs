// MIT/Apache2 License

// Note: There is currently a weird abort in Wine for yaww. I believe the source of the abort
//       to be in this file. Audit it later.

use crate::directive::Directive;
//use parking::{Parker, Unparker};
use std::{
    alloc::{self, Layout},
    any::{Any, TypeId},
    cell::RefCell,
    future::Future,
    hint::{spin_loop, unreachable_unchecked},
    marker::PhantomData,
    mem,
    pin::Pin,
    process::abort,
    ptr::{self, NonNull},
    sync::atomic::{AtomicU8, Ordering},
    task::{Context, Poll, Waker},
    thread::{self, Thread},
};

/// A task that represents an ongoing operation being done by the GUI server.
pub struct Task<T: Any + Send + Sync + 'static> {
    // a pointer to the heap object where the task is stored
    real_task: NonNull<()>,
    _phantom: PhantomData<Option<T>>,
}

// SAFETY: all components of a Task are Send and Sync, since it's essentially an Arc<T + Waker + Directive>
unsafe impl<T: Any + Send + Sync + 'static> Send for Task<T> {}
unsafe impl<T: Any + Send + Sync + 'static> Sync for Task<T> {}

/// The server-facing side of the task.
pub(crate) struct ServerTask {
    real_task: NonNull<()>,
}

unsafe impl Send for ServerTask {}

/// The header of a task that describes its current state.
struct TaskHeader {
    // the scheme we use here:
    //  00 - task has not yet been waited on
    //  01 - task is currently parking a thread via the Parker struct
    //  10 - task is currently pending a future via the AtomicWaker struct
    //  11 - task has been completed and the output field is valid
    //
    // for 00, 01, and 10, it can be assumed that there are two pointers to the task. for 11, the pointer is
    // exclusive
    // if the 3rd bit is toggled, it means the allocation has been abandoned and the server may drop it if it
    // wants to
    // the 4th bit is toggled while we're actually writing to and from the directive/output, and basically
    // functions as a spinlock mutex
    header: AtomicU8,
    // the vtable for the task
    vtable: TaskVtable,
}

enum TaskState {
    NotWaited,
    Parked,
    Pending,
    Complete,
}

impl TaskHeader {
    // spin until the reference is guaranteed to be exclusive
    #[inline]
    fn wait_exclusive(&self) {
        log::trace!("Entering wait_exclusive spin loop");

        let mut past_state;
        loop {
            past_state = self.header.load(Ordering::SeqCst);
            if past_state & 0b1000 == 0 {
                break;
            }

            spin_loop();
        }

        self.header.store(past_state | 0b1000, Ordering::SeqCst);
        log::trace!("We have now acquired exclusive access to the waker/parker");
    }

    #[inline]
    fn unset_exclusive(&self) {
        self.header.fetch_and(!0b1000, Ordering::SeqCst);
        log::trace!(
            "Dropping exclusive access to the waker/parker, new state is {}",
            self.header.load(Ordering::SeqCst)
        );
    }

    // tell whether or not we're abandoned
    #[inline]
    fn is_abandoned(&self) -> bool {
        self.header.load(Ordering::Acquire) & 0b0100 != 0
    }

    // get the current state of the task from the first two bits
    #[inline]
    fn state(&self) -> TaskState {
        let state = self.header.load(Ordering::Acquire) & 0b0011;
        match state {
            0b00 => TaskState::NotWaited,
            0b01 => TaskState::Parked,
            0b10 => TaskState::Pending,
            0b11 => TaskState::Complete,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    // set this task to be parked
    #[inline]
    fn park(&self) -> bool {
        let state = self.header.fetch_or(0b01, Ordering::SeqCst) & 0b0011;
        log::trace!("Parked, old state was {}", state);
        log::trace!("new state is {}", self.header.load(Ordering::SeqCst));
        if state == 0b10 {
            // parking on a pending state is an invalid operation
            // this means we've fucked up, abort
            panic!("Unable to run sync operations on an async task");
        } else if state == 0b11 {
            true
        } else {
            false
        }
    }

    // set this task to be pending
    #[inline]
    fn pend(&self) {
        let state = self.header.fetch_or(0b10, Ordering::SeqCst) & 0b0011;
        if state == 0b01 {
            // parking on a pending state is an invalid operation
            // this means we've fucked up, abort
            panic!("Unable to run async operations on a sync task");
        }
    }
}

/// A pointer to the fields of the task.
struct TaskLayout {
    // layout of the entire thing
    layout: Layout,
    // layout into the task where the directive would be stored
    directive_offset: usize,
    // layout into the task where the output would be stored
    output_offset: usize,
    // layout into the task where the parker would be stored
    parker_offset: usize,
    // layout into the task where the waker would be stored
    waker_offset: usize,
}

struct RawTask<T> {
    // the header for the task
    header: *const TaskHeader,
    // the directive used for the task
    directive: *const Directive,
    // the output that the task is expected to produce
    output: *mut T,
    // the thread that the task uses to unpark itself if thread blocking mode is selected
    parker: *mut Thread,
    //parker: *mut Unparker,
    // the waker that the task uses if nonblocking mode is used
    waker: *mut Waker,
}

#[derive(Copy, Clone)]
struct TaskVtable {
    park: unsafe fn(*const ()),
    pend: unsafe fn(*const (), Waker),
    complete: unsafe fn(*const (), *const (), TypeId),
    directive: unsafe fn(*const ()) -> *const Directive,
    output: unsafe fn(*const ()) -> *const (),
    drop: unsafe fn(*const (), bool),
}

impl<T> RawTask<T>
where
    T: Any + Send + Sync + 'static,
{
    const VTABLE: TaskVtable = TaskVtable {
        park: Self::park,
        pend: Self::pend,
        complete: Self::complete,
        directive: Self::directive,
        output: Self::output,
        drop: Self::drop,
    };

    fn allocate(directive: Directive) -> NonNull<()> {
        // create the layout of the task
        let layout = Self::task_layout();

        log::trace!(
            "Allocating a task with type {} and with size {}",
            std::any::type_name::<T>(),
            layout.layout.size()
        );

        // allocate the memory for the task
        let pointer = match NonNull::new(unsafe { alloc::alloc(layout.layout) as *mut () }) {
            Some(p) => p,
            None => abort(), // oom
        };

        // get the pointers to the various fields
        let task = Self::from_ptr(pointer.as_ptr());

        // write in the fields of the task
        // this is all safe since, for now, we hold the only reference to this allocation
        unsafe {
            (task.header as *mut TaskHeader).write(TaskHeader {
                header: AtomicU8::new(0),
                vtable: Self::VTABLE,
            });

            (task.directive as *mut Directive).write(directive);
        }

        pointer
    }

    // set this task up to be awoken via a parker/unparker pair
    unsafe fn park(ptr: *const ()) {
        log::trace!("Parking thread in waiting for task");
        let this = Self::from_ptr(ptr);

        // first, clarify that we are parking the thread, and also reserve an exclusive spot
        let header = unsafe { &*this.header };
        if header.park() {
            // we're already finished, no need to park
            return;
        }
        header.wait_exclusive();

        // get a handle to our thread and write it to the "parker" slot, then park the thread
        // this is entirely safe, because the above call to make_exclusive() ensures that we are the only thread
        // with access to this slot
        let current_thread = thread::current();
        unsafe { this.parker.write(current_thread) };
        //let (parker, unparker) = parking::pair();
        //unsafe { this.parker.write(unparker) };

        // now, park the thread
        header.unset_exclusive();

        loop {
            // we call the parker in a tight loop
            // since the thread may spuriously wake up, we check if the task is completed
            // after each wakeup
            if header.header.load(Ordering::SeqCst) & 0b11 == 0b11 {
                break;
            }

            thread::park();
            //parker.park();
        }
    }

    // set this task up to be awoken via a waker
    unsafe fn pend(ptr: *const (), waker: Waker) {
        // basically the same as above, but we don't park the thread
        let this = Self::from_ptr(ptr);
        let header = unsafe { &*this.header };
        header.wait_exclusive();
        header.pend();
        unsafe { this.waker.write(waker) };
        header.unset_exclusive();
    }

    // set this task to be completed
    unsafe fn complete(ptr: *const (), value: *const (), type_id: TypeId) {
        log::trace!("Completing task...");

        if cfg!(debug_assertions) {
            // panic if the type id doesn't match up
            assert_eq!(TypeId::of::<T>(), type_id);
        } else {
            let _ = type_id;
        }

        let this = Self::from_ptr(ptr);

        // copy the output from the value pointer to the output pointer in our task
        // this is safe because only two threads ever access the directive or the output:
        //  1). the owning thread, who only accesses it after we set our task to complete
        //  2). the gui thread, the only accesses it before we set our task to complete
        unsafe {
            ptr::copy_nonoverlapping(
                value as *const u8,
                this.output as *mut u8,
                mem::size_of::<T>(),
            )
        };

        // now that our output is ready, we set the state to complete and check the remaining values
        let header = unsafe { &*this.header };
        let old_state = header.header.fetch_or(0b11, Ordering::SeqCst) & 0b11;
        log::trace!("State prior to completion was {}", old_state);
        match old_state {
            0b01 => {
                // we need to unpark the old parker
                header.wait_exclusive();
                let unparker = unsafe { ptr::read(this.parker) };
                log::trace!("Unparking thread...");
                unparker.unpark();
                header.unset_exclusive();
            }
            0b10 => {
                // we need to wake the old waker
                header.wait_exclusive();
                let waker = unsafe { ptr::read(this.waker) };
                waker.wake();
                header.unset_exclusive();
            }
            _ => (), // the only other option is 0b00, which indicates the task was never waited
        }
    }

    unsafe fn directive(ptr: *const ()) -> *const Directive {
        Self::from_ptr(ptr).directive
    }

    unsafe fn output(ptr: *const ()) -> *const () {
        Self::from_ptr(ptr).output as *const _ as *const ()
    }

    unsafe fn drop(ptr: *const (), expected_drop: bool) {
        log::trace!("Deleting task with type {}", std::any::type_name::<T>());
        let this = Self::from_ptr(ptr);
        let layout = Self::task_layout();

        // note: relaxed is okay, if we're being dropped there's only one reference
        let state = unsafe { &*this.header }.header.load(Ordering::Relaxed) & 0b11;
        if state == 0b11 {
            // if we are complete, but the drop is unexpected (read: the user dropped completed task for some
            // reason), drop the output
            if !expected_drop {
                unsafe { ptr::drop_in_place(this.output) };
            }
        } else {
            if expected_drop {
                // the task was abandoned by the user before the GUI thread got to it, dealloc the task
                unsafe { ptr::drop_in_place(this.directive as *mut Directive) };
            } else {
                // the task was abandoned by the user after the GUI thread got to it, dealloc the output
                unsafe { ptr::drop_in_place(this.output) };
            }
        }

        if state == 0b01 {
            // we got dropped mid-park (how?), dealloc the unparker
            unsafe { ptr::drop_in_place(this.parker) };
        } else if state == 0b10 {
            // we got dropped mid-pend, dealloc the waker
            unsafe { ptr::drop_in_place(this.waker) };
        }

        // now, we dealloc the memory containing the task
        unsafe { alloc::dealloc(ptr as *mut u8, layout.layout) };
    }

    #[inline]
    fn from_ptr(ptr: *const ()) -> Self {
        let layout = Self::task_layout();
        let p = ptr as *const u8;

        Self {
            header: p as *const TaskHeader,
            directive: unsafe { p.add(layout.directive_offset) as *mut Directive },
            output: unsafe { p.add(layout.output_offset) as *mut T },
            parker: unsafe { p.add(layout.parker_offset) as *mut Thread },
            waker: unsafe { p.add(layout.waker_offset) as *mut Waker },
        }
    }

    // figure out what our layout looks like
    #[inline]
    fn task_layout() -> TaskLayout {
        let header = Layout::new::<TaskHeader>();
        let directive = Layout::new::<Directive>();
        let output = Layout::new::<T>();
        let parker = Layout::new::<Thread>();
        let waker = Layout::new::<Waker>();

        // we combine the directive and output into a union to save space, since we'll never
        // have both
        let d_o_union = unsafe {
            Layout::from_size_align_unchecked(
                directive.size().max(output.size()),
                directive.align().max(output.align()),
            )
        };

        // same for the parker and waker
        let p_w_union = unsafe {
            Layout::from_size_align_unchecked(
                parker.size().max(waker.size()),
                parker.align().max(waker.align()),
            )
        };

        // now we combine them all into a comprehensive layout
        // header goes first so we can cast the pointer to a *const TaskHeader to check state
        let layout = header;
        let (layout, directive_offset) = layout.extend(d_o_union).unwrap();
        let (layout, parker_offset) = layout.extend(p_w_union).unwrap();

        TaskLayout {
            layout,
            directive_offset,
            output_offset: directive_offset,
            parker_offset,
            waker_offset: parker_offset,
        }
    }
}

// Create the task/servertask pair
// This is unsafe because the task assumes the server task has been sent to the server before anything is done
// with it.
pub(crate) unsafe fn create_task<T: Any + Send + Sync + 'static>(
    directive: Directive,
) -> (Task<T>, ServerTask) {
    let real_task = RawTask::<T>::allocate(directive);
    let task = Task {
        real_task,
        _phantom: PhantomData,
    };
    let server_task = ServerTask { real_task };
    (task, server_task)
}

impl<T: Any + Send + Sync + 'static> Task<T> {
    /// Wait for the task to resolve to a real result by parking the current thread and waiting.
    #[inline]
    pub fn wait(self) -> T {
        // first, park the thread if necessary
        let header: NonNull<TaskHeader> = self.real_task.cast();
        let header = unsafe { header.as_ref() };

        unsafe { (header.vtable.park)(self.real_task.as_ptr()) };

        // now that the thread is unparked, read in the result and return
        let res = unsafe { ptr::read((header.vtable.output)(self.real_task.as_ptr()) as *const T) };
        // drop the task, now that we have the output
        unsafe { (header.vtable.drop)(self.real_task.as_ptr(), true) };
        res
    }
}

impl<T: Any + Send + Sync + 'static> Future for Task<T> {
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let header: NonNull<TaskHeader> = self.as_ref().real_task.cast();
        let header = unsafe { header.as_ref() };

        // read from the header, if the state is 0b11 the state is complete and we can return
        let state = header.header.load(Ordering::SeqCst) & 0b11;
        match state {
            0b11 => {
                let res = unsafe {
                    ptr::read((header.vtable.output)(self.as_ref().real_task.as_ptr()) as *const T)
                };
                // drop the task, now that we have the output
                unsafe { (header.vtable.drop)(self.as_ref().real_task.as_ptr(), true) };
                Poll::Ready(res)
            }
            0b00 => {
                // we need to register a waker with the task
                unsafe {
                    (header.vtable.pend)(self.as_ref().real_task.as_ptr(), cx.waker().clone())
                }
                Poll::Pending
            }
            0b10 => {
                // the task is being polled before completion
                Poll::Pending
            }
            _ => {
                panic!("Invalid state")
            }
        }
    }
}

impl<T: Any + Send + Sync + 'static> Drop for Task<T> {
    #[inline]
    fn drop(&mut self) {
        let header: NonNull<TaskHeader> = self.real_task.cast();
        let header = unsafe { header.as_ref() };

        // set the task to be abandoned
        let state = header.header.fetch_or(0b100, Ordering::SeqCst) & 0b11;

        // if the state given is 0b11, the task is completed and it's our responsibility to clean up the
        // memory
        unsafe { (header.vtable.drop)(self.real_task.as_ptr(), false) };
    }
}

impl ServerTask {
    /// Get the raw pointer to the inner task.
    #[inline]
    pub fn as_ptr(&self) -> NonNull<()> {
        self.real_task
    }

    #[inline]
    pub unsafe fn from_ptr(p: NonNull<()>) -> Self {
        Self { real_task: p }
    }

    /// Tell whether or not this task is abandoned.
    #[inline]
    pub fn is_abandoned(&self) -> bool {
        unsafe { (&*(self.real_task.as_ptr() as *const TaskHeader)) }
            .header
            .load(Ordering::SeqCst)
            & 0b100
            != 0
    }

    /// Read the directive from this given task.
    ///
    /// unsafe because misuse can create untracked directive clones
    #[inline]
    pub unsafe fn directive(&self) -> Option<Directive> {
        if self.is_abandoned() {
            None
        } else {
            let header: NonNull<TaskHeader> = self.real_task.cast();
            let header = unsafe { header.as_ref() };

            Some(unsafe { ptr::read((header.vtable.directive)(self.real_task.as_ptr())) })
        }
    }

    /// Destroy this task if it was abandoned.
    #[inline]
    pub fn destroy(self) {
        unsafe {
            ((&*(self.real_task.as_ptr() as *const TaskHeader))
                .vtable
                .drop)(self.real_task.as_ptr(), true)
        };
        mem::forget(self);
    }

    /// Complete this task with the output provided.
    #[inline]
    pub fn complete<T: Any + Send + Sync + 'static>(self, value: T) {
        let header: NonNull<TaskHeader> = self.real_task.cast();
        let header = unsafe { header.as_ref() };

        // complete the internal task with our given value
        unsafe {
            (header.vtable.complete)(
                self.real_task.as_ptr(),
                &value as *const T as *const (),
                TypeId::of::<T>(),
            )
        };
        // we don't need to dealloc memory, it's the task's problem now
    }
}

impl Drop for ServerTask {
    #[inline]
    fn drop(&mut self) {
        // if the task is abandoned, it's our job to clean it up
        if self.is_abandoned() {
            // false means we don't dealloc the directive
            unsafe {
                ((&*(self.real_task.as_ptr() as *const TaskHeader))
                    .vtable
                    .drop)(self.real_task.as_ptr(), false)
            };
        }
    }
}

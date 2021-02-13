// MIT/Apache2 License

use super::Directive;
use crate::{
    brush::Brush,
    cursor::Cursor,
    icon::Icon,
    menu::Menu,
    task::ServerTask,
    window::{ExtendedWindowStyle, Window, WindowStyle},
    window_class::ClassStyle,
    window_data::WindowData,
    wndproc::yaww_wndproc,
    Rectangle,
};
use std::{
    ffi::{CStr, CString},
    mem,
    process::abort,
    ptr,
};
use winapi::{ctypes::c_int, shared::windef::RECT, um::winuser};

impl Directive {
    #[inline]
    pub(crate) fn process(self, window_data: &WindowData, task: ServerTask) {
        macro_rules! complete_with_rectangle {
            ($task: expr, $window: expr, $fname: ident) => {{
                let mut rect = Rectangle {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                };
                $task.complete::<crate::Result<Rectangle>>(
                    if unsafe {
                        winuser::$fname(
                            $window.as_ptr().as_ptr().cast(),
                            &mut rect as *mut Rectangle as *mut _,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some(stringify!($fname))))
                    } else {
                        Ok(rect)
                    },
                )
            }};
        }

        match self {
            Directive::SetEventHandler(event_handler) => {
                // set the event handler
                *window_data
                    .event_handler
                    .try_borrow_mut()
                    .expect("Tried to set event handler while processing event") =
                    event_handler.into_inner();
                task.complete::<()>(());
            }
            Directive::BeginWait => {
                *window_data.waiter.borrow_mut() = Some(task);
            }
            Directive::RegisterClass {
                style,
                icon,
                small_icon,
                cursor,
                background,
                class_name,
                menu_name,
            } => {
                task.complete::<crate::Result>(register_class(
                    style,
                    icon,
                    small_icon,
                    cursor,
                    background,
                    &*class_name,
                    menu_name.as_deref(),
                ));
            }
            Directive::CreateWindow {
                class_name,
                window_name,
                style,
                extended_style,
                x,
                y,
                width,
                height,
                parent,
                menu,
            } => task.complete::<crate::Result<Window>>(create_window(
                window_data,
                &*class_name,
                window_name.as_deref(),
                style,
                extended_style,
                x,
                y,
                width,
                height,
                parent,
                menu,
            )),
            Directive::ShowWindow { window, command } => {
                unsafe { winuser::ShowWindow(window.as_ptr().as_ptr().cast(), command.bits()) };
                task.complete::<()>(());
            }
            Directive::CloseWindow(window) => task.complete::<crate::Result>(
                if unsafe { winuser::CloseWindow(window.as_ptr().as_ptr().cast()) } == 0 {
                    Err(crate::Error::win32_error(Some("CloseWindow")))
                } else {
                    Ok(())
                },
            ),
            Directive::GetClientRect(window) => {
                complete_with_rectangle!(task, window, GetClientRect);
            }
            Directive::GetDesktopWindow => {
                let res = unsafe { winuser::GetDesktopWindow() };
                // if this fails, something is seriously fucked up
                task.complete::<Window>(
                    Window::from_ptr(res.cast()).expect("Desktop window does not exist"),
                );
            }
            Directive::GetWindowRect(window) => {
                complete_with_rectangle!(task, window, GetWindowRect);
            }
            Directive::GetParent(window) => {
                task.complete::<Option<Window>>(Window::from_ptr(unsafe {
                    winuser::GetParent(window.as_ptr().as_ptr().cast()).cast()
                }));
            }
            Directive::GetWindowText(window) => {
                let textlen =
                    unsafe { winuser::GetWindowTextLengthA(window.as_ptr().as_ptr().cast()) };
                let mut buffer = Vec::<u8>::with_capacity(textlen as usize + 1);
                let textlen = unsafe {
                    winuser::GetWindowTextA(
                        window.as_ptr().as_ptr().cast(),
                        buffer.as_mut_ptr() as *mut _,
                        textlen + 1,
                    )
                };

                task.complete::<Option<CString>>(if textlen <= 0 {
                    None
                } else {
                    unsafe { buffer.set_len(textlen as usize - 1) };
                    CString::new(buffer).ok()
                });
            }
            Directive::MoveWindow {
                window,
                x,
                y,
                width,
                height,
                repaint,
            } => task.complete::<crate::Result>(
                if unsafe {
                    winuser::MoveWindow(
                        window.as_ptr().as_ptr().cast(),
                        x,
                        y,
                        width,
                        height,
                        if repaint { 1 } else { 0 },
                    )
                } == 0
                {
                    Err(crate::Error::win32_error(Some("MoveWindow")))
                } else {
                    Ok(())
                },
            ),
            _ => unreachable!(),
        }
    }
}

#[inline]
fn register_class(
    style: ClassStyle,
    icon: Option<Icon>,
    small_icon: Option<Icon>,
    cursor: Option<Cursor>,
    background: Option<Brush>,
    class_name: &CStr,
    menu_name: Option<&CStr>,
) -> crate::Result {
    let icon = match icon {
        Some(icon) => unsafe { icon.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };
    let small_icon = match small_icon {
        Some(icon) => unsafe { icon.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };
    let cursor = match cursor {
        Some(cursor) => unsafe { cursor.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };
    let background = match background {
        Some(background) => unsafe { background.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };
    let class = winuser::WNDCLASSEXA {
        cbSize: mem::size_of::<winuser::WNDCLASSEXA>() as _,
        style: style.bits(),
        lpfnWndProc: Some(yaww_wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: ptr::null_mut(),
        hIcon: icon.cast(),
        hCursor: cursor.cast(),
        hbrBackground: background.cast(),
        lpszMenuName: match menu_name {
            Some(mn) => mn.as_ptr().cast(),
            None => ptr::null(),
        },
        lpszClassName: class_name.as_ptr().cast(),
        hIconSm: small_icon.cast(),
    };
    if unsafe { winuser::RegisterClassExA(&class) } == 0 {
        Err(crate::Error::win32_error(Some("RegisterClassExA")))
    } else {
        Ok(())
    }
}

#[inline]
fn create_window(
    window_data: &WindowData,
    class_name: &CStr,
    menu_name: Option<&CStr>,
    style: WindowStyle,
    extended_style: ExtendedWindowStyle,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    parent: Option<Window>,
    menu: Option<Menu>,
) -> crate::Result<Window> {
    // since we're creating a window, we increment the window count
    window_data
        .window_count
        .set(window_data.window_count.get() + 1);

    let parent = match parent {
        Some(p) => unsafe { p.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };
    let menu = match menu {
        Some(m) => unsafe { m.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };

    let res = unsafe {
        winuser::CreateWindowExA(
            extended_style.bits(),
            class_name.as_ptr(),
            match menu_name {
                Some(mn) => mn.as_ptr(),
                None => ptr::null(),
            },
            style.bits(),
            x,
            y,
            width,
            height,
            parent.cast(),
            menu.cast(),
            ptr::null_mut(),
            window_data as *const WindowData as *const _ as *mut _,
        )
    };
    match Window::from_ptr(res.cast()) {
        Some(win) => Ok(win),
        None => Err(crate::Error::win32_error(Some("CreateWindowExA"))),
    }
}

// MIT/Apache2 License

//! Process a message into a directive.

use super::{Directive, KeyType, Provider, Response};
use crate::{
    brush::Brush,
    cursor::Cursor,
    icon::Icon,
    menu::Menu,
    refcell::RefCell,
    window::{ClassStyle, ExtendedWindowStyle, ShowWindowCommand, Window, WindowStyle},
    wndproc::{yaww_wndproc, WindowData},
};
use std::{
    ffi::CStr,
    mem,
    ptr::{self, NonNull},
};
use winapi::{
    ctypes::c_int,
    um::{libloaderapi, winuser},
};

#[inline]
pub(crate) fn process_directive(
    directive: Directive,
    provider: &mut Provider,
    window_data: &RefCell<WindowData>,
) -> crate::Result<Response> {
    match directive {
        Directive::SetEventHandler(event) => {
            // SAFETY: we're single-threaded so we'll never actually
            //         have more than one access at once
            let mut wd = window_data.borrow_mut();
            wd.event_handler = event;
            Ok(Response::Empty)
        }
        Directive::RegisterClass {
            class_name,
            style,
            icon,
            small_icon,
            cursor,
            background,
            menu_name,
        } => register_class(
            provider,
            &class_name,
            style,
            icon,
            small_icon,
            cursor,
            background,
            menu_name.as_deref(),
        ),
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
        } => create_window(
            provider,
            window_data,
            &class_name,
            &window_name,
            style,
            extended_style,
            x,
            y,
            width,
            height,
            parent,
            menu,
        ),
        Directive::ShowWindow { window, command } => show_window(provider, window, command),
        _ => Ok(Response::Empty),
    }
}

#[inline]
fn register_class(
    provider: &mut Provider,
    class_name: &CStr,
    style: ClassStyle,
    icon: Option<Icon>,
    small_icon: Option<Icon>,
    cursor: Option<Cursor>,
    background: Option<Brush>,
    menu_name: Option<&CStr>,
) -> crate::Result<Response> {
    let icon_ptr = match icon {
        Some(icon) => provider.translate(icon)?.as_ptr(),
        None => ptr::null_mut(),
    };
    let small_icon_ptr = match small_icon {
        Some(small_icon) => provider.translate(small_icon)?.as_ptr(),
        None => icon_ptr,
    };
    let cursor_ptr = match cursor {
        Some(cursor) => provider.translate(cursor)?,
        // SAFETY: on any sane Win32 system, IDC_ARROW is predefined
        None => unsafe {
            NonNull::new_unchecked(
                winuser::LoadCursorA(ptr::null_mut(), winuser::IDC_ARROW.cast()).cast(),
            )
        },
    };
    let brush_ptr = match background {
        Some(brush) => provider.translate(brush)?,
        None => unsafe { NonNull::new_unchecked((winuser::COLOR_WINDOW as usize + 1) as *mut ()) },
    };

    // create the window class struct
    let cls = winuser::WNDCLASSEXA {
        cbSize: mem::size_of::<winuser::WNDCLASSEXA>() as _,
        style: style.bits(),
        lpfnWndProc: Some(yaww_wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: unsafe { libloaderapi::GetModuleHandleW(ptr::null()) },
        hIcon: icon_ptr.cast(),
        hCursor: cursor_ptr.cast().as_ptr(),
        hbrBackground: brush_ptr.cast().as_ptr(),
        lpszMenuName: match menu_name {
            Some(menu_name) => menu_name.as_ptr(),
            None => ptr::null(),
        },
        lpszClassName: class_name.as_ptr(),
        hIconSm: small_icon_ptr.cast(),
    };

    // register the window class
    if unsafe { winuser::RegisterClassExA(&cls) } == 0 {
        Err(crate::Error::win32_error(Some("RegisterClassExA")))
    } else {
        Ok(Response::Empty)
    }
}

#[inline]
fn create_window(
    provider: &mut Provider,
    data_pointer: &RefCell<WindowData>,
    class_name: &CStr,
    window_name: &CStr,
    style: WindowStyle,
    extended_style: ExtendedWindowStyle,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    parent: Option<Window>,
    menu: Option<Menu>,
) -> crate::Result<Response> {
    // increment the window count. we set up the GUI thread loop to exit if the window count
    // reaches zero during a wait cycle, and the window decrements the count when it is
    // destroyed
    let mut window_data = data_pointer.borrow_mut();
    window_data.window_count += 1;
    log::debug!("Window count is now {}", window_data.window_count);
    mem::drop(window_data);

    let res = unsafe {
        winuser::CreateWindowExA(
            extended_style.bits(),
            class_name.as_ptr(),
            window_name.as_ptr(),
            style.bits(),
            x,
            y,
            width,
            height,
            match parent {
                Some(parent) => provider.translate(parent)?.as_ptr().cast(),
                None => ptr::null_mut(),
            },
            match menu {
                Some(parent) => provider.translate(parent)?.as_ptr().cast(),
                None => ptr::null_mut(),
            },
            unsafe { libloaderapi::GetModuleHandleW(ptr::null()) },
            data_pointer as *const RefCell<WindowData> as *mut RefCell<WindowData> as *mut _,
        )
    };

    match NonNull::new(res) {
        None => Err(crate::Error::win32_error(Some("CreateWindowExA"))),
        Some(res) => Ok(Response::Window(
            provider.create_key(res.cast(), KeyType::Window)?,
        )),
    }
}

#[inline]
fn show_window(
    provider: &mut Provider,
    window: Window,
    command: ShowWindowCommand,
) -> crate::Result<Response> {
    unsafe { winuser::ShowWindow(provider.translate(window)?.cast().as_ptr(), command.bits()) };
    Ok(Response::Empty)
}

// MIT/Apache2 License

//! Process a message into a directive.

use super::{Directive, KeyType, Provider, RasterOperation, Response};
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
    shared::minwindef::BOOL,
    um::{libloaderapi, wingdi, winuser},
};

#[inline]
pub(crate) fn process_directive(
    directive: Directive,
    window_data: &WindowData,
) -> crate::Result<Response> {
    match directive {
        Directive::SetEventHandler(event) => {
            // SAFETY: we're single-threaded so we'll never actually
            //         have more than one access at once
            let mut wd = window_data.exclusive.borrow_mut();
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
            window_data,
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
        Directive::MoveWindow {
            window,
            x,
            y,
            width,
            height,
            repaint,
        } => move_window(window_data, window, x, y, width, height, repaint),
        Directive::ShowWindow { window, command } => show_window(window_data, window, command),
        Directive::DeleteObject(obj) => {
            let obj = window_data.provider.borrow_mut().translate(obj)?;
            unsafe { wingdi::DeleteObject(obj.cast().as_ptr()) };
            window_data.provider.borrow_mut().remove_key(obj);
            Ok(Response::Empty)
        }
        Directive::CreateCompatibleDc(dc) => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            let res_dc = unsafe { wingdi::CreateCompatibleDC(dc.cast().as_ptr()) };
            match NonNull::new(res_dc) {
                Some(res_dc) => Ok(Response::Dc(
                    window_data
                        .provider
                        .borrow_mut()
                        .create_key(res_dc.cast(), KeyType::Dc, false)?,
                )),
                None => Err(crate::Error::win32_error(Some("CreateCompatibleDc"))),
            }
        }
        Directive::DeleteDc(dc) => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::DeleteDC(dc.cast().as_ptr()) };
            Ok(Response::Empty)
        }
        Directive::SelectObject { dc, object } => {
            let mut provider = window_data.provider.borrow_mut();
            let dc = provider.translate(dc)?;
            let obj = provider.translate(object)?;
            mem::drop(provider);

            let res_obj = unsafe { wingdi::SelectObject(dc.cast().as_ptr(), obj.cast().as_ptr()) };
            let res_obj = match NonNull::new(res_obj) {
                Some(res_obj) => window_data
                    .provider
                    .borrow_mut()
                    .create_key(res_obj.cast(), KeyType::GdiObject, true)
                    .unwrap(),

                None => return Err(crate::Error::win32_error(Some("SelectObject"))),
            };

            Ok(Response::GdiObject(res_obj))
        }
        Directive::SetPixel { dc, x, y, color } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::SetPixel(dc.cast().as_ptr(), x, y, color.colorref()) };
            Ok(Response::Empty)
        }
        Directive::MoveTo { dc, x, y } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::MoveToEx(dc.cast().as_ptr(), x, y, ptr::null_mut()) };
            Ok(Response::Empty)
        }
        Directive::LineTo { dc, x, y } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::LineTo(dc.cast().as_ptr(), x, y) };
            Ok(Response::Empty)
        }
        Directive::Rectangle {
            dc,
            left,
            top,
            right,
            bottom,
        } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::Rectangle(dc.cast().as_ptr(), left, top, right, bottom) };
            Ok(Response::Empty)
        }
        Directive::Bezier { dc, points } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::PolyBezier(dc.cast().as_ptr(), points.as_ptr(), points.len() as _) };
            Ok(Response::Empty)
        }
        Directive::Polygon { dc, points } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::Polygon(dc.cast().as_ptr(), points.as_ptr(), points.len() as _) };
            Ok(Response::Empty)
        }
        Directive::Polyline { dc, points } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::Polyline(dc.cast().as_ptr(), points.as_ptr(), points.len() as _) };
            Ok(Response::Empty)
        }
        Directive::Ellipse {
            dc,
            left,
            top,
            right,
            bottom,
        } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe { wingdi::Ellipse(dc.cast().as_ptr(), left, top, right, bottom) };
            Ok(Response::Empty)
        }
        Directive::RoundRect {
            dc,
            left,
            top,
            right,
            bottom,
            ellipse_width,
            ellipse_height,
        } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe {
                wingdi::RoundRect(
                    dc.cast().as_ptr(),
                    left,
                    top,
                    right,
                    bottom,
                    ellipse_width,
                    ellipse_height,
                )
            };
            Ok(Response::Empty)
        }
        Directive::Chord {
            dc,
            rect_left,
            rect_top,
            rect_right,
            rect_bottom,
            line_x1,
            line_y1,
            line_x2,
            line_y2,
        } => {
            let dc = window_data.provider.borrow_mut().translate(dc)?;
            unsafe {
                wingdi::Chord(
                    dc.cast().as_ptr(),
                    rect_left,
                    rect_top,
                    rect_right - rect_left,
                    rect_bottom - rect_top,
                    line_x1,
                    line_y1,
                    line_x2,
                    line_y2,
                )
            };
            Ok(Response::Empty)
        }
        Directive::BitBlt {
            src,
            dest,
            srcx,
            srcy,
            destx,
            desty,
            width,
            height,
            op,
        } => {
            let mut provider = window_data.provider.borrow_mut();
            let src = provider.translate(src)?;
            let dest = provider.translate(dest)?;
            mem::drop(provider);

            let op = match op {
                RasterOperation::SrcCopy => wingdi::SRCCOPY,
            };

            unsafe {
                wingdi::BitBlt(
                    dest.cast().as_ptr(),
                    destx,
                    desty,
                    width,
                    height,
                    src.cast().as_ptr(),
                    srcx,
                    srcy,
                    op,
                )
            };
            Ok(Response::Empty)
        }
        _ => Ok(Response::Empty),
    }
}

#[inline]
fn register_class(
    window_data: &WindowData,
    class_name: &CStr,
    style: ClassStyle,
    icon: Option<Icon>,
    small_icon: Option<Icon>,
    cursor: Option<Cursor>,
    background: Option<Brush>,
    menu_name: Option<&CStr>,
) -> crate::Result<Response> {
    let mut provider = window_data.provider.borrow_mut();

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

    mem::drop(provider);

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
    window_data: &WindowData,
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
    let mut ex = window_data.exclusive.borrow_mut();
    ex.window_count += 1;
    log::debug!("Window count is now {}", ex.window_count);
    mem::drop(ex);

    let mut provider = window_data.provider.borrow_mut();
    let parent = match parent {
        Some(parent) => provider.translate(parent)?.as_ptr().cast(),
        None => ptr::null_mut(),
    };
    let menu = match menu {
        Some(parent) => provider.translate(parent)?.as_ptr().cast(),
        None => ptr::null_mut(),
    };

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
            parent,
            menu,
            unsafe { libloaderapi::GetModuleHandleW(ptr::null()) },
            window_data as *const WindowData as *mut WindowData as *mut _,
        )
    };

    match NonNull::new(res) {
        None => Err(crate::Error::win32_error(Some("CreateWindowExA"))),
        Some(res) => Ok(Response::Window(provider.create_key(
            res.cast(),
            KeyType::Window,
            false,
        )?)),
    }
}

#[inline]
fn show_window(
    window_data: &WindowData,
    window: Window,
    command: ShowWindowCommand,
) -> crate::Result<Response> {
    let mut provider = window_data.provider.borrow_mut();
    let window = provider.translate(window)?.cast().as_ptr();
    mem::drop(provider);

    unsafe { winuser::ShowWindow(window, command.bits()) };
    Ok(Response::Empty)
}

#[inline]
fn move_window(
    window_data: &WindowData,
    window: Window,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    repaint: bool,
) -> crate::Result<Response> {
    let mut provider = window_data.provider.borrow_mut();
    let window = provider.translate(window)?.cast().as_ptr();
    mem::drop(provider);

    if unsafe { winuser::MoveWindow(window, x, y, width, height, repaint as BOOL) } == 0 {
        Err(crate::Error::win32_error(Some("MoveWindow")))
    } else {
        Ok(Response::Empty)
    }
}

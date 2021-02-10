// MIT/Apache2 License

use super::Directive;
use crate::{task::ServerTask, window_data::WindowData, wndproc::yaww_wndproc};
use std::{mem, ptr};
use winapi::um::winuser;

impl Directive {
    #[inline]
    pub(crate) fn process(self, window_data: &WindowData, task: ServerTask) {
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
            Directive::RegisterClass {
                style,
                icon,
                small_icon,
                cursor,
                background,
                class_name,
                menu_name,
            } => {
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
                let res = if unsafe { winuser::RegisterClassExA(&class) } == 0 {
                    Err(crate::Error::win32_error(Some("RegisterClassExA")))
                } else {
                    Ok(())
                };

                task.complete::<crate::Result>(res);
            }
            _ => unreachable!(),
        }
    }
}

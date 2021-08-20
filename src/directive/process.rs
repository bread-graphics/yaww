// MIT/Apache2 License

use super::Directive;
use crate::{
    bitmap::Bitmap,
    brush::Brush,
    cursor::Cursor,
    dc::{BitBltOp, Dc},
    gdiobj::{GdiObject, StockObject},
    glrc::{GlProc, Glrc},
    icon::Icon,
    menu::Menu,
    monitor::{Monitor, MonitorInfo},
    pen::{Pen, PenStyle},
    server::YawwController,
    window::{ExtendedWindowStyle, Window, WindowStyle},
    window_class::ClassStyle,
    wndproc::{yaww_wndproc, HandleAndSubclass},
    Rectangle,
};
use breadthread::{AddOrRemovePtr, Completer};
use std::{
    ffi::{CStr, CString},
    mem::{self, MaybeUninit},
    process::abort,
    ptr::{self, NonNull},
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{BOOL, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HDC, HMONITOR, HWND, LPRECT, POINT, RECT},
    },
    um::{libloaderapi, wingdi, winuser},
};

impl Directive {
    #[inline]
    pub(crate) fn process<'evh, C: Completer>(
        self,
        controller: &YawwController<'evh>,
        completer: &mut C,
    ) -> AddOrRemovePtr {
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
            Directive::GetMonitors => {
                // function to iterate over monitors
                unsafe extern "system" fn monitor_enum_proc(
                    monitor: HMONITOR,
                    _hdc: HDC,
                    rect: LPRECT,
                    target: LPARAM,
                ) -> BOOL {
                    // we can't feasibly panic here, set up a bomb to abort
                    struct Bomb;

                    impl Drop for Bomb {
                        #[cold]
                        fn drop(&mut self) {
                            abort();
                        }
                    }

                    let _bomb = Bomb;

                    // SAFETY: "target" will always be an "*mut Vec<MonitorInfo>"
                    let target: *mut Vec<MonitorInfo> =
                        mem::transmute::<isize, *mut Vec<MonitorInfo>>(target);
                    let target = &mut *target;
                    let rect = &*rect;

                    let monitor_info = MonitorInfo {
                        monitor: match Monitor::from_ptr(monitor.cast()) {
                            Some(m) => m,
                            None => {
                                log::error!("Monitor should never be null!");
                                mem::forget(_bomb);
                                return 0;
                            }
                        },
                        x: rect.left,
                        y: rect.top,
                        width: rect.right - rect.left,
                        height: rect.bottom - rect.top,
                    };

                    target.push(monitor_info);

                    // disarm the bomb
                    mem::forget(_bomb);

                    1
                }

                let mut monitor_infos: Vec<MonitorInfo> = vec![];
                completer.complete::<crate::Result<Vec<MonitorInfo>>>(
                    if unsafe {
                        winuser::EnumDisplayMonitors(
                            ptr::null_mut(),
                            ptr::null_mut(),
                            Some(monitor_enum_proc),
                            &mut monitor_infos as *mut Vec<MonitorInfo> as isize,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("EnumDisplayMonitors")))
                    } else {
                        Ok(monitor_infos)
                    },
                );
            }
            Directive::GetDefaultMonitor => {
                const ZERO_POINT: POINT = POINT { x: 0, y: 0 };
                let monitor = unsafe {
                    winuser::MonitorFromPoint(ZERO_POINT, winuser::MONITOR_DEFAULTTOPRIMARY)
                };
                let monitor = Monitor::from_ptr(monitor.cast());
                completer.complete::<crate::Result<Monitor>>(
                    monitor.ok_or_else(|| crate::Error::win32_error(Some("MonitorFromPoint"))),
                );
                if let Some(monitor) = monitor {
                    return AddOrRemovePtr::AddPtr(monitor.into_raw());
                }
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
                completer.complete::<crate::Result>(register_class(
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
                base_class,
                window_name,
                style,
                extended_style,
                x,
                y,
                width,
                height,
                parent,
                menu,
            } => {
                let res = create_window(
                    controller,
                    &*class_name,
                    base_class.as_deref(),
                    window_name.as_deref(),
                    style,
                    extended_style,
                    x,
                    y,
                    width,
                    height,
                    parent,
                    menu,
                );

                let aorp = match &res {
                    Ok(win) => AddOrRemovePtr::AddPtr(win.into_raw()),
                    Err(_) => AddOrRemovePtr::DoNothing,
                };

                completer.complete::<crate::Result<Window>>(res);

                return aorp;
            }
            Directive::ShowWindow { window, command } => {
                unsafe { winuser::ShowWindow(window.as_ptr().as_ptr().cast(), command.bits()) };
                completer.complete::<()>(());
            }
            Directive::CloseWindow(window) => {
                completer.complete::<crate::Result>(
                    if unsafe { winuser::CloseWindow(window.as_ptr().as_ptr().cast()) } == 0 {
                        Err(crate::Error::win32_error(Some("CloseWindow")))
                    } else {
                        Ok(())
                    },
                );
                return AddOrRemovePtr::RemovePtr(window.into_raw());
            }
            Directive::GetClientRect(window) => {
                complete_with_rectangle!(completer, window, GetClientRect);
            }
            Directive::GetDesktopWindow => {
                let res = unsafe { winuser::GetDesktopWindow() };
                let win = Window::from_ptr(res.cast()).expect("Desktop window does not exist");
                // if this fails, something is seriously messed up
                completer.complete::<Window>(win);
                return AddOrRemovePtr::AddPtr(win.into_raw());
            }
            Directive::GetWindowRect(window) => {
                complete_with_rectangle!(completer, window, GetWindowRect);
            }
            Directive::GetParent(window) => {
                let win = Window::from_ptr(unsafe {
                    winuser::GetAncestor(window.as_ptr().as_ptr().cast(), winuser::GA_PARENT).cast()
                });
                completer.complete::<Option<Window>>(win);
                if let Some(win) = win {
                    return AddOrRemovePtr::AddPtr(win.into_raw());
                }
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

                completer.complete::<Option<CString>>(if textlen <= 0 {
                    None
                } else {
                    unsafe { buffer.set_len(textlen as usize - 1) };
                    CString::new(buffer).ok()
                });
            }
            Directive::InvalidateRect {
                window,
                rect,
                erase,
            } => completer.complete::<crate::Result>(
                if unsafe {
                    winuser::InvalidateRect(
                        window.as_ptr().as_ptr().cast(),
                        match rect {
                            Some(ref r) => r as *const Rectangle as *const _,
                            None => ptr::null(),
                        },
                        if erase { 1 } else { 0 },
                    )
                } == 0
                {
                    Err(crate::Error::win32_error(Some("InvalidateRect")))
                } else {
                    Ok(())
                },
            ),
            Directive::IsChild { parent, child } => completer.complete::<bool>(
                unsafe {
                    winuser::IsChild(
                        parent.as_ptr().as_ptr().cast(),
                        child.as_ptr().as_ptr().cast(),
                    )
                } != 0,
            ),
            Directive::IsZoomed(window) => completer.complete::<bool>(
                unsafe { winuser::IsZoomed(window.as_ptr().as_ptr().cast()) } != 0,
            ),
            Directive::MoveWindow {
                window,
                x,
                y,
                width,
                height,
                dont_move,
                dont_resize,
                repaint,
            } => completer.complete::<crate::Result>(
                if unsafe {
                    winuser::SetWindowPos(
                        window.as_ptr().as_ptr().cast(),
                        ptr::null_mut(),
                        x,
                        y,
                        width,
                        height,
                        {
                            let mut flags = winuser::SWP_NOZORDER;
                            if dont_move {
                                flags |= winuser::SWP_NOMOVE;
                            }
                            if dont_resize {
                                flags |= winuser::SWP_NOSIZE;
                            }
                            if !repaint {
                                flags |= winuser::SWP_NOREDRAW;
                            }
                            flags
                        },
                    )
                } == 0
                {
                    Err(crate::Error::win32_error(Some("MoveWindow")))
                } else {
                    Ok(())
                },
            ),
            Directive::SetParent { window, new_parent } => {
                let res = unsafe {
                    winuser::SetParent(
                        window.as_ptr().as_ptr().cast(),
                        match new_parent {
                            Some(np) => np.as_ptr().as_ptr().cast(),
                            None => ptr::null_mut(),
                        },
                    )
                };
                let win = Window::from_ptr(res.cast());

                completer.complete::<crate::Result<Window>>(match win {
                    None => Err(crate::Error::win32_error(Some("SetParent"))),
                    Some(res) => Ok(res),
                });

                if let Some(win) = win {
                    return AddOrRemovePtr::AddPtr(win.into_raw());
                }
            }
            Directive::SetWindowText { window, text } => completer.complete::<crate::Result>(
                if unsafe {
                    winuser::SetWindowTextA(window.as_ptr().as_ptr().cast(), text.as_ptr())
                } == 0
                {
                    Err(crate::Error::win32_error(Some("SetWindowText")))
                } else {
                    Ok(())
                },
            ),
            Directive::UpdateWindow(window) => completer.complete::<crate::Result>(
                if unsafe { winuser::UpdateWindow(window.as_ptr().as_ptr().cast()) } == 0 {
                    Err(crate::Error::win32_error(Some("UpdateWindow")))
                } else {
                    Ok(())
                },
            ),
            Directive::SelectObject { dc, obj } => {
                let res = unsafe {
                    wingdi::SelectObject(dc.as_ptr().as_ptr().cast(), obj.as_ptr().as_ptr().cast())
                };
                let obj = GdiObject::from_ptr(res.cast());
                completer.complete::<crate::Result<GdiObject>>(match obj {
                    Some(o) => Ok(o),
                    None => Err(crate::Error::win32_error(Some("SelectObject"))),
                });
                if let Some(obj) = obj {
                    return AddOrRemovePtr::AddPtr(obj.into_raw());
                }
            }
            Directive::ReleaseDc { window, dc } => {
                unsafe {
                    winuser::ReleaseDC(window.as_ptr().as_ptr().cast(), dc.as_ptr().as_ptr().cast())
                };
                completer.complete::<()>(());
            }
            Directive::CreateCompatibleDc(dc) => {
                let res = unsafe { wingdi::CreateCompatibleDC(dc.as_ptr().as_ptr().cast()) };
                let obj = Window::from_ptr(res.cast());
                completer.complete::<crate::Result<Dc>>(match obj {
                    Some(o) => Ok(o),
                    None => Err(crate::Error::win32_error(Some("CreateCompatibleDc"))),
                });
                if let Some(obj) = obj {
                    return AddOrRemovePtr::AddPtr(obj.into_raw());
                }
            }
            Directive::DeleteDc(dc) => {
                unsafe { wingdi::DeleteDC(dc.as_ptr().as_ptr().cast()) };
                completer.complete::<()>(());
                return AddOrRemovePtr::RemovePtr(dc.into_raw());
            }
            Directive::SetPixel { dc, x, y, color } => {
                completer.complete::<crate::Result>(
                    if (unsafe {
                        wingdi::SetPixel(dc.as_ptr().as_ptr().cast(), x, y, color.colorref())
                    } as i32)
                        < 0
                    {
                        Err(crate::Error::win32_error(Some("SetPixel")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::MoveTo { dc, x, y } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::MoveToEx(dc.as_ptr().as_ptr().cast(), x, y, ptr::null_mut())
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("MoveToEx")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::LineTo { dc, x, y } => {
                completer.complete::<crate::Result>(
                    if unsafe { wingdi::LineTo(dc.as_ptr().as_ptr().cast(), x, y) } == 0 {
                        Err(crate::Error::win32_error(Some("LineTo")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::Rectangle {
                dc,
                left,
                top,
                right,
                bottom,
            } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::Rectangle(dc.as_ptr().as_ptr().cast(), left, top, right, bottom)
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("Rectangle")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::RoundRect {
                dc,
                left,
                top,
                right,
                bottom,
                width,
                height,
            } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::RoundRect(
                            dc.as_ptr().as_ptr().cast(),
                            left,
                            top,
                            right,
                            bottom,
                            width,
                            height,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("RoundRect")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::Arc {
                dc,
                rect_left,
                rect_top,
                rect_right,
                rect_bottom,
                arc_start_x,
                arc_start_y,
                arc_end_x,
                arc_end_y,
            } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::Arc(
                            dc.as_ptr().as_ptr().cast(),
                            rect_left,
                            rect_top,
                            rect_right,
                            rect_bottom,
                            arc_start_x,
                            arc_start_y,
                            arc_end_x,
                            arc_end_y,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("Arc")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::Ellipse {
                dc,
                left,
                top,
                right,
                bottom,
            } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::Ellipse(dc.as_ptr().as_ptr().cast(), left, top, right, bottom)
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("Ellipse")))
                    } else {
                        Ok(())
                    },
                );
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
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::Chord(
                            dc.as_ptr().as_ptr().cast(),
                            rect_left,
                            rect_top,
                            rect_right,
                            rect_bottom,
                            line_x1,
                            line_y1,
                            line_x2,
                            line_y2,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("Chord")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::BezierCurve { dc, points } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::PolyBezier(
                            dc.as_ptr().as_ptr().cast(),
                            points.as_ptr() as *const _,
                            points.len() as _,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("PolyBezier")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::Polygon { dc, points } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::Polygon(
                            dc.as_ptr().as_ptr().cast(),
                            points.as_ptr() as *const _,
                            points.len() as _,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("Polygon")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::Polyline { dc, points } => {
                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::Polyline(
                            dc.as_ptr().as_ptr().cast(),
                            points.as_ptr() as *const _,
                            points.len() as _,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("Polyline")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::SwapBuffers(dc) => completer.complete::<crate::Result>(
                if unsafe { wingdi::SwapBuffers(dc.as_ptr().as_ptr().cast()) } == 0 {
                    Err(crate::Error::win32_error(Some("SwapBuffers")))
                } else {
                    Ok(())
                },
            ),
            Directive::CreatePen {
                style,
                width,
                color,
            } => {
                let style = match style {
                    PenStyle::Solid => wingdi::PS_SOLID,
                    PenStyle::Dash => wingdi::PS_DASH,
                    PenStyle::Dot => wingdi::PS_DOT,
                    PenStyle::DashDot => wingdi::PS_DASHDOT,
                    PenStyle::DashDotDot => wingdi::PS_DASHDOTDOT,
                    PenStyle::Null => wingdi::PS_NULL,
                    PenStyle::InsideFrame => wingdi::PS_INSIDEFRAME,
                };
                let res = unsafe { wingdi::CreatePen(style as _, width, color.colorref()) };
                let pen = Pen::from_ptr(res.cast());

                completer.complete::<crate::Result<Pen>>(match pen {
                    Some(p) => Ok(p),
                    None => Err(crate::Error::win32_error(Some("CreatePen"))),
                });

                if let Some(pen) = pen {
                    return AddOrRemovePtr::AddPtr(pen.into_raw());
                }
            }
            Directive::CreateSolidBrush(color) => {
                let res = unsafe { wingdi::CreateSolidBrush(color.colorref()) };
                let brush = Brush::from_ptr(res.cast());

                completer.complete::<crate::Result<Brush>>(match brush {
                    Some(b) => Ok(b),
                    None => Err(crate::Error::win32_error(Some("CreateSolidBrush"))),
                });

                if let Some(brush) = brush {
                    return AddOrRemovePtr::AddPtr(brush.into_raw());
                }
            }
            Directive::GetStockObject(so) => {
                let ty = match so {
                    StockObject::NullBrush => wingdi::NULL_BRUSH as c_int,
                };
                let res = unsafe { wingdi::GetStockObject(ty) };
                let obj = GdiObject::from_ptr(res.cast());

                completer.complete::<Option<GdiObject>>(obj);

                if let Some(obj) = obj {
                    return AddOrRemovePtr::AddPtr(obj.into_raw());
                }
            }
            Directive::DeleteObject { obj } => {
                unsafe { wingdi::DeleteObject(obj.as_ptr().as_ptr().cast()) };
                completer.complete::<()>(());
                return AddOrRemovePtr::RemovePtr(obj.into_raw());
            }
            Directive::CreateCompatibleBitmap { dc, width, height } => {
                let bitmap = unsafe {
                    wingdi::CreateCompatibleBitmap(dc.as_ptr().as_ptr().cast(), width, height)
                };
                match GdiObject::from_ptr(bitmap.cast()) {
                    Some(obj) => {
                        completer.complete::<crate::Result<Bitmap>>(Ok(obj));
                        return AddOrRemovePtr::AddPtr(obj.into_raw());
                    }
                    None => {
                        completer.complete::<crate::Result<Bitmap>>(Err(
                            crate::Error::win32_error(Some("CreateCompatibleBitmap")),
                        ));
                    }
                }
            }
            Directive::SetPixels {
                dc,
                origin_x,
                origin_y,
                width,
                pixels,
            } => {
                pixels
                    .into_iter()
                    .fold((origin_x, origin_y), move |(x, y), pixel| {
                        unsafe { wingdi::SetPixelV(dc.as_ptr().as_ptr().cast(), x, y, pixel) };

                        match x + 1 {
                            x if x >= width => (0, y + 1),
                            x => (x, y),
                        }
                    });

                completer.complete::<()>(());
            }
            Directive::BitBlt {
                src,
                dst,
                src_x,
                src_y,
                dst_x,
                dst_y,
                width,
                height,
                op,
            } => {
                let op = match op {
                    BitBltOp::Blackness => wingdi::BLACKNESS,
                    BitBltOp::CaptureBlt => wingdi::CAPTUREBLT,
                    BitBltOp::DstInvert => wingdi::DSTINVERT,
                    BitBltOp::MergeCopy => wingdi::MERGECOPY,
                    BitBltOp::MergePaint => wingdi::MERGEPAINT,
                    BitBltOp::NoMirrorBitmap => wingdi::NOMIRRORBITMAP,
                    BitBltOp::NotSrcCopy => wingdi::NOTSRCCOPY,
                    BitBltOp::NotSrcErase => wingdi::NOTSRCERASE,
                    BitBltOp::PatCopy => wingdi::PATCOPY,
                    BitBltOp::PatInvert => wingdi::PATINVERT,
                    BitBltOp::PatPaint => wingdi::PATPAINT,
                    BitBltOp::SrcAnd => wingdi::SRCAND,
                    BitBltOp::SrcCopy => wingdi::SRCCOPY,
                    BitBltOp::SrcErase => wingdi::SRCERASE,
                    BitBltOp::SrcInvert => wingdi::SRCINVERT,
                    BitBltOp::SrcPaint => wingdi::SRCPAINT,
                    BitBltOp::Whiteness => wingdi::WHITENESS,
                };

                completer.complete::<crate::Result>(
                    if unsafe {
                        wingdi::BitBlt(
                            src.as_ptr().as_ptr().cast(),
                            src_x,
                            src_y,
                            width,
                            height,
                            dst.as_ptr().as_ptr().cast(),
                            dst_x,
                            dst_y,
                            op,
                        )
                    } == 0
                    {
                        Err(crate::Error::win32_error(Some("BitBlt")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::CreateWglContext(dc) => {
                let res = Glrc::from_ptr(
                    unsafe { wingdi::wglCreateContext(dc.as_ptr().as_ptr().cast()) }.cast(),
                );
                completer.complete::<crate::Result<Glrc>>(match res {
                    Some(res) => Ok(res),
                    None => Err(crate::Error::win32_error(Some("wglCreateContext"))),
                });
                if let Some(res) = res {
                    return AddOrRemovePtr::AddPtr(res.into_raw());
                }
            }
            Directive::MakeWglCurrent { dc, rc } => {
                let dc = match dc {
                    Some(dc) => unsafe { dc.as_ptr() }.as_ptr(),
                    None => ptr::null_mut(),
                };
                let rc = match rc {
                    Some(rc) => unsafe { rc.as_ptr() }.as_ptr(),
                    None => ptr::null_mut(),
                };

                completer.complete::<crate::Result>(
                    if unsafe { wingdi::wglMakeCurrent(dc.cast(), rc.cast()) } == 0 {
                        Err(crate::Error::win32_error(Some("wglMakeCurrent")))
                    } else {
                        Ok(())
                    },
                );
            }
            Directive::DestroyWglContext(rc) => {
                completer.complete::<crate::Result>(
                    if unsafe { wingdi::wglDeleteContext(rc.as_ptr().as_ptr().cast()) } == 0 {
                        Err(crate::Error::win32_error(Some("wglDeleteContext")))
                    } else {
                        Ok(())
                    },
                );
                return AddOrRemovePtr::RemovePtr(rc.into_raw());
            }
            Directive::GetWglProcAddress(name) => {
                completer.complete::<Option<GlProc>>(
                    match NonNull::new(unsafe { wingdi::wglGetProcAddress(name.as_ptr()) }) {
                        Some(glproc) => Some(unsafe { GlProc::new(glproc.cast()) }),
                        None => None,
                    },
                );
            }
            Directive::ChoosePixelFormat { dc, pixel_format } => {
                let pixel_format: wingdi::PIXELFORMATDESCRIPTOR = pixel_format.into_owned().into();
                let res = unsafe {
                    wingdi::ChoosePixelFormat(dc.as_ptr().as_ptr().cast(), &pixel_format)
                };
                completer.complete::<crate::Result<c_int>>(if res == 0 {
                    Err(crate::Error::win32_error(Some("ChoosePixelFormat")))
                } else {
                    Ok(res)
                });
            }
            Directive::SetPixelFormat {
                dc,
                format_id,
                pixel_format,
            } => {
                let pixel_format: wingdi::PIXELFORMATDESCRIPTOR = pixel_format.into_owned().into();
                let res = unsafe {
                    wingdi::SetPixelFormat(dc.as_ptr().as_ptr().cast(), format_id, &pixel_format)
                };
                completer.complete::<crate::Result>(if res == 0 {
                    Err(crate::Error::win32_error(Some("SetPixelFormat")))
                } else {
                    Ok(())
                });
            }
        }

        AddOrRemovePtr::DoNothing
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
fn create_window<'evh>(
    window_data: &YawwController<'evh>,
    class_name: &CStr,
    base_class: Option<&CStr>,
    window_name: Option<&CStr>,
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
    window_data.increment_window_count();

    let parent = match parent {
        Some(p) => unsafe { p.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };
    let menu = match menu {
        Some(m) => unsafe { m.as_ptr().as_ptr() },
        None => ptr::null_mut(),
    };

    // if we're doing the subclass thing, do it here
    let subclass = if let Some(base_class) = base_class {
        let mut cls: MaybeUninit<winuser::WNDCLASSEXA> = MaybeUninit::uninit();
        if unsafe {
            winuser::GetClassInfoExA(
                ptr::null_mut(),
                base_class.as_ptr().cast(),
                cls.as_mut_ptr(),
            )
        } == 0
        {
            // try again, but with the local module as the scope
            if unsafe {
                winuser::GetClassInfoExA(
                    libloaderapi::GetModuleHandleA(ptr::null()),
                    base_class.as_ptr().cast(),
                    cls.as_mut_ptr(),
                )
            } == 0
            {
                return Err(crate::Error::win32_error(Some("GetClassInfoExA")));
            }
        }

        let cls = unsafe { MaybeUninit::assume_init(cls) };
        let subclass = cls.lpfnWndProc.expect("wndproc is an empty pointer?");
        Some(subclass)
    } else {
        None
    };

    let handle = window_data.handle().clone();
    let has = HandleAndSubclass {
        handle: Box::new(handle),
        subclass,
    };
    let has = Box::new(has);

    let res = unsafe {
        winuser::CreateWindowExA(
            extended_style.bits(),
            class_name.as_ptr(),
            match window_name {
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
            Box::into_raw(has) as *mut _,
        )
    };
    match Window::from_ptr(res.cast()) {
        Some(win) => Ok(win),
        None => Err(crate::Error::win32_error(Some("CreateWindowExA"))),
    }
}

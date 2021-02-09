// MIT/Apache2 License

use std::{ffi::CStr, ops, os::raw::c_char};
use yaww::{
    window::{ClassStyle, ExtendedWindowStyle, ShowWindowCommand, WindowStyle},
    Color, Event, GuiThread, PenStyle, Result,
};

const CLASS_NAME: ConstCstr<'static> = ConstCstr::new(&*b"examples_basic_class\0");
const WINDOW_NAME: ConstCstr<'static> = ConstCstr::new(&*b"YAWW Example\0");

fn main() -> Result {
    env_logger::Builder::new()
        .filter(Some("yaww"), log::LevelFilter::Debug)
        .init();

    let gui_thread = GuiThread::new();
    gui_thread.register_class(
        &*CLASS_NAME,
        None,
        ClassStyle::empty(),
        None,
        None,
        None,
        None,
    )?;
    let window = gui_thread.create_window(
        &*CLASS_NAME,
        &*WINDOW_NAME,
        WindowStyle::OVERLAPPED_WINDOW,
        ExtendedWindowStyle::empty(),
        0,
        0,
        640,
        400,
        None,
        None,
    )?;
    window.show(&gui_thread, ShowWindowCommand::SHOW)?;

    let gt_clone = gui_thread.clone();
    gui_thread.set_event_handler(move |ev| {
        println!("{:?}", &ev);
        match ev {
            Event::Paint { dc, .. } => {
                // create a pen
                let pen = gt_clone.create_pen(PenStyle::Dash, 50, Color::from_rgb(0, 0, 0))?;
                let brush = gt_clone.create_solid_brush(Color::from_rgb(255, 0, 0))?;

                let hold_pen = dc.select_object(&gt_clone, pen)?;
                let hold_brush = dc.select_object(&gt_clone, brush)?;
                //dc.move_to(&gt_clone, 0, 0)?;
                //dc.line_to(&gt_clone, 600, 400)?;
                //dc.rectangle(&gt_clone, 30, 30, 300, 300)?;
                for _ in 0..100 {
                    dc.set_pixel(
                        &gt_clone,
                        fastrand::i32(0..600),
                        fastrand::i32(0..400),
                        Color::from_rgb(0, 0, 0),
                    )?;
                }
                dc.select_object(&gt_clone, hold_pen)?;
                dc.select_object(&gt_clone, hold_brush)?;

                pen.delete(&gt_clone)?;
                brush.delete(&gt_clone)?;
            }
            _ => (),
        }

        Ok(())
    })?;

    window.invalidate_rect(&gui_thread, None, true)?;
    gui_thread.wait()?;

    Ok(())
}

struct ConstCstr<'a> {
    inner: &'a [u8],
}

impl<'a> ConstCstr<'a> {
    #[inline]
    const fn new(c: &'a [u8]) -> Self {
        Self { inner: c }
    }
}

impl<'a> ops::Deref for ConstCstr<'a> {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &CStr {
        CStr::from_bytes_with_nul(self.inner).unwrap()
    }
}

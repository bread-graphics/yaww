// MIT/Apache2 License

use const_cstr::const_cstr;
use yaww::{
    brush::DEFAULT_BRUSH,
    window::{ExtendedWindowStyle, ShowWindowCommand, WindowStyle},
    window_class::ClassStyle,
    pen::PenStyle,
    Event, Color, GuiThread, Result,
};

const_cstr! {
    CLASS_NAME = "yaww-example-class";
    WINDOW_NAME = "YAWW Basic Example";
}

fn main() -> Result {
    env_logger::Builder::new()
        .filter(Some("yaww"), log::LevelFilter::Debug)
        .init();

    // create the gui thread
    let gt = GuiThread::new();

    // register a window class
    // note: register_class, along with most other functions in yaww, return a Task that can
    //       either be waited on or awaited
    gt.register_class(
        CLASS_NAME.as_cstr(),
        None,
        ClassStyle::empty(),
        None,
        None,
        None,
        Some(DEFAULT_BRUSH),
    )?
    .wait()?;

    // create the window proper
    let window = gt
        .create_window(
            CLASS_NAME.as_cstr(),
            Some(WINDOW_NAME.as_cstr().into()),
            WindowStyle::OVERLAPPED_WINDOW,
            ExtendedWindowStyle::CLIENT_EDGE,
            0,
            0,
            640,
            400,
            None,
            None,
        )?
        .wait()?;
    window.show(&gt, ShowWindowCommand::SHOW)?.wait();
    window.update_window(&gt)?.wait()?;

    // create a pen
    let pen = gt.create_pen(PenStyle::Solid, 5, Color::from_rgb(0, 0, 0))?.wait()?;

    // set up an event handler
    gt.set_event_handler(move |gt, ev| {
        println!("{:?}", &ev);

        match ev {
            Event::KeyDown { .. } => window
                .move_window(gt, 20, 20, 600, 600, true)
                .unwrap()
                .wait()
                .unwrap(),
            Event::Paint { dc, .. } => {
                // paint a few shapes on the window
                let hold_pen = dc.select_object(gt, pen).unwrap().wait().unwrap();
//                dc.ellipse(gt, 20, 20, 300, 300).unwrap().wait().unwrap();
                dc.line_to(gt, 100, 100).unwrap().wait().unwrap();
                dc.select_object(gt, hold_pen).unwrap().wait().unwrap();
            }
            Event::Quit => {
                pen.delete(gt).unwrap().wait();
            }
            _ => (),
        }
    })?.wait();

    gt.main_loop()?.wait();
    Ok(())
}

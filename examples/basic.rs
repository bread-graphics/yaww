// MIT/Apache2 License

use const_cstr::const_cstr;
use yaww::{
    brush::DEFAULT_BRUSH,
    window::{ShowWindowCommand, ExtendedWindowStyle, WindowStyle},
    window_class::ClassStyle,
    GuiThread, Result,
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
    let window = gt.create_window(
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

    Ok(())
}

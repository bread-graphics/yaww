// MIT/Apache2 License

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum Event {
    Quit,
    Close,
}

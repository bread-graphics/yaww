// MIT/Apache2 License

use crate::directive::Directive;
use std::any::Any;
use orphan_crippler::{Sender, Receiver, two};

pub type Task<T> = Receiver<T>; 
pub(crate) type ServerTask = Sender<Directive>;

#[inline]
pub(crate) fn create_task<T: Any + Send>(dir: Directive) -> (Task<T>, ServerTask) {
    let (send, recv) = two(dir);
    (recv, send)
}

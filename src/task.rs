// MIT/Apache2 License

use crate::directive::Directive;
use orphan_crippler::{two, Receiver, Sender};
use std::any::Any;

pub type Task<T> = Receiver<T>;

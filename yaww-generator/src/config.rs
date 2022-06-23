// MIT/Apache2 License

use std::{borrow::Cow, rc::Rc};

#[derive(Debug, serde::Deserialize)]
pub struct Config<'a> {
    pub directives: Directives<'a>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Directives<'a> {
    pub modules: Vec<Cow<'a, str>>,
    pub ignore: Vec<Cow<'a, str>>,
    pub actual_bools: Vec<Cow<'a, str>>,
}

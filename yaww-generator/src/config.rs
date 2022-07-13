// MIT/Apache2 License

use std::{borrow::Cow, rc::Rc, collections::HashMap};

#[derive(Debug, serde::Deserialize)]
pub struct Config<'a> {
    pub directives: Directives<'a>,
    pub real_arr_type: HashMap<Cow<'a, str>, RealArrayType<'a>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Directives<'a> {
    pub modules: Vec<Cow<'a, str>>,
    pub ignore: Vec<Cow<'a, str>>,
    pub actual_bools: Vec<Cow<'a, str>>,
    pub dont_merge: Vec<Cow<'a, str>>,
    pub stub_struct: Vec<Cow<'a, str>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RealArrayType<'a> {
    pub fname: Cow<'a, str>,
    pub real_ty: Cow<'a, str>,
}

//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use std::{borrow::Cow, collections::HashMap, rc::Rc};

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

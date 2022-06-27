// MIT/Apache2 License

use crate::{fits_globs, swwriteln, types::ty::PtrClass, State, any_res, all_res};
use anyhow::{anyhow, Context, Result};
use heck::{AsSnakeCase, AsUpperCamelCase};
use std::{
    fmt::{self, Write},
    ops::Index,
};
use take_mut::take;
use windows_metadata::reader::{self, ParamFlags};

mod structure;
pub use structure::{Field, ResolveLater};

mod item;
pub use item::Item;

mod ty;
pub use ty::Type;

#[derive(Debug, PartialEq, Eq)]
pub enum Special {
    Wparam,
    Lparam,
    Lresult,
}

impl Special {
    pub fn win32_name(&self) -> &'static str {
        match self {
            Special::Wparam => "WPARAM",
            Special::Lparam => "LPARAM",
            Special::Lresult => "LRESULT",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Special::Wparam => "Wparam",
            Special::Lparam => "Lparam",
            Special::Lresult => "Lresult",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Constant {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i128),
    String(String),
    Boolean(bool),
    Float(f64),
}

// dont care about nan
impl Eq for Value {}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "0x{:X}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Float(fo) => write!(f, "{}", fo),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: &'static str,
    pub return_type: Option<Type>,
    pub params: Vec<Param>,
    pub merged_params: Vec<usize>,
    pub namespace: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Param {
    pub name: &'static str,
    pub ty: Type,
    pub variation: Variation,
    pub optional: bool,

    // other param names that were merged into this one
    pub merged: Vec<&'static str>,
}

impl Param {
    pub fn new(name: &'static str, ty: Type, variation: Variation, optional: bool) -> Self {
        Self {
            name,
            ty,
            variation,
            optional,
            merged: Vec::new(),
        }
    }

    pub fn involves_void(&self, state: &mut State<'_>) -> Result<bool> {
        self.ty.involves_void(state)
    }

    /// Setup some uninit memory for this param.
    fn setup_uninit_memory(&self, params: &[Param], state: &mut State<'_>) -> Result<()> {
        match self.ty.unwrap_mut_ref()? {
            Type::OsString | Type::String => {
                // setup a buffer of the size given by the "n" or "max" parameter
                for param in params.iter().rev() {
                    if matches!(param.ty.unwrap_mut_ref()?, Type::Primitive("i32" | "u32"))
                        && (param.name.starts_with('n')
                            || param.name.contains("max")
                            || param.name.starts_with('c')
                            || param.name.contains("cch"))
                    {
                        // this param will be our length
                        swwriteln!(
                            state,
                            "let mut {} = Vec::<{}>::with_capacity({} as usize + 1);",
                            AsSnakeCase(self.name),
                            match self.ty.unwrap_mut_ref()? {
                                Type::String => "u8",
                                Type::OsString => "u16",
                                _ => unreachable!(),
                            },
                            AsSnakeCase(param.name),
                        )?;
                        swwriteln!(
                            state,
                            "let {} = {}.as_mut_ptr();",
                            self.name,
                            AsSnakeCase(self.name),
                        )?;

                        return Ok(());
                    }
                }

                return Err(anyhow!(
                    "Did not find len parameter for str out param {}",
                    self.name
                ));
            }
            _ => {
                swwriteln!(
                    state,
                    "let mut {} = mem::MaybeUninit::zeroed();",
                    AsSnakeCase(self.name),
                )?;
                swwriteln!(
                    state,
                    "let {} = {}.as_mut_ptr();",
                    self.name,
                    AsSnakeCase(self.name),
                )?;
            }
        }

        Ok(())
    }

    fn setup_input(&self, state: &mut State<'_>) -> Result<()> {
        let mut ty = &self.ty;
        let mut make_mutable = false;
        if matches!(self.variation, Variation::InOut) {
            ty = ty.unwrap_mut_ref()?;
            make_mutable = true;
        }

        let expr = ty.input_expression(&FieldName(&self.name).to_string(), self.optional, state)?;
        swwriteln!(
            state,
            "let {}{} = {};",
            if make_mutable { "mut " } else { "" },
            Sanitize(&self.name),
            expr,
        )?;

        self.setup_input_merged_parameters(state)?;

        Ok(())
    }

    /// When setting up input, call this too to set up merged parameters.
    fn setup_input_merged_parameters(&self, state: &mut State<'_>) -> Result<()> {
        match &self.ty {
            Type::Slice(_) => {
                // the first merged parameter will be our length
                let length_name = self.merged.first().unwrap();
                swwriteln!(
                    state,
                    "let {} = {}.len() as _;",
                    Sanitize(length_name),
                    FieldName(self.name),
                )?;
            }
            Type::Callback {
                fn_type,
                input_ptr_ty,
                ..
            } => {
                // allow the function to be mutably borrowed if necessary
                if let FnType::FnMut = fn_type {
                    swwriteln!(state, "let mut {0} = {0};", FieldName(self.name))?;
                }

                let ptr_name = self.merged.first().unwrap();
                swwrite!(state, "let {} = ", Sanitize(ptr_name))?;

                match input_ptr_ty {
                    PtrClass::Wparam => swwrite!(state, "unsafe {{ Wparam::from_ptr(")?,
                    PtrClass::Lparam => swwrite!(state, "unsafe {{ Lparam::from_ptr(")?,
                    _ => {}
                }

                // the actual function takes a pointer to the inner
                // function and casts it to call it
                match fn_type {
                    FnType::FnMut => {
                        swwrite!(state, "(&mut {}) as *mut _ as *mut _", FieldName(self.name),)?
                    }
                }

                if let PtrClass::Wparam | PtrClass::Lparam = input_ptr_ty {
                    swwrite!(state, ") }}")?;
                }

                swwriteln!(state, ";")?;
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_returned_value(&self, params: &[Param], state: &mut State<'_>) -> Result<()> {
        let input_expr = match self.variation {
            Variation::InOut => Sanitize(&self.name).to_string(),
            Variation::Output => format!("unsafe {{ {}.assume_init() }}", Sanitize(&self.name)),
            _ => unreachable!(),
        };

        let expr = self
            .ty
            .unwrap_mut_ref()?
            .returnable_expr(&input_expr, params, state)?;
        swwriteln!(state, "let {} = {};", FieldName(self.name), expr,)?;

        Ok(())
    }

    /// Get the generics necessary for this item.
    fn generics(&self, state: &mut State<'_>) -> Result<Option<String>> {
        Ok(match &self.ty {
            Type::Callback {
                name,
                params,
                fn_type,
                input_ptr_ty,
                ret_ty,
            } => {
                let mut genbound = format!(
                    "{}: {}(",
                    AsUpperCamelCase(name),
                    match fn_type {
                        FnType::FnMut => "FnMut",
                    }
                );

                for (i, param) in params
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| p.ptr_class() != Some(*input_ptr_ty))
                {
                    let ip = param.param_position(true, state)?;
                    genbound.push_str(&ip);
                    if i != params.len() - 1 {
                        genbound.push_str(", ");
                    }
                }

                genbound.push(')');

                if let Some(ret_ty) = ret_ty {
                    let rp = ret_ty.return_position(state)?;
                    genbound.push_str(" -> ");
                    genbound.push_str(&rp);
                }

                Some(genbound)
            }
            _ => None,
        })
    }
}

/// Merge various parameters together.
///
/// This algorithm merges refs and their associated lengths to create
/// slices, and functions with pointers to make callbacks.
fn merge_params(
    params: &mut [Param],
    merged_params: &mut Vec<usize>,
    state: &mut State<'_>,
) -> Result<()> {
    enum MergeInfo {
        MakeSlice,
        MakeCallback { input_ptr_ty: PtrClass },
    }

    // try to find a param to be merged
    while let Some((index, param, merge_index, merge_info)) = params
        .iter()
        .enumerate()
        .filter(|(i, _)| !merged_params.contains(i))
        .rev()
        .find_map(|(index, param)| {
            let mut mergable_params = params
                .iter()
                .enumerate()
                .filter(|(i, _)| !merged_params.contains(i))
                .filter(|(_, p)| !fits_globs(p.name, &state.dont_merge_globs));

            match &param.ty {
                Type::Ref(ty) => {
                    // string references are never merged
                    if ty.is_str_ref() || ty.is_os_str_ref() {
                        return None;
                    }

                    // look for a potential size for this value, it might be a slice
                    mergable_params
                        .rfind(|(_, param)| {
                            likely_slice_count(param.name)
                                && matches!(param.ty, Type::Primitive("i32" | "u32"))
                        })
                        .map(move |(size_index, _)| {
                            (index, param, size_index, MergeInfo::MakeSlice)
                        })
                }
                Type::Item(item) => match &**item {
                    Item::FunctionType {
                        name,
                        params: func_params,
                        return_type,
                    } => {
                        // look for a pointer type that is:
                        // - a: one of *const void, lparam or wparam
                        // - b: also in the params list
                        mergable_params.rev().find_map(|(merge_index, tparam)| {
                            let ptr_class = tparam.ty.ptr_class()?;

                            if func_params
                                .iter()
                                .filter_map(|p| p.ty.ptr_class())
                                .any(|p| p == ptr_class)
                            {
                                Some((
                                    index,
                                    param,
                                    merge_index,
                                    MergeInfo::MakeCallback {
                                        input_ptr_ty: ptr_class,
                                    },
                                ))
                            } else {
                                None
                            }
                        })
                    }
                    _ => None,
                },
                _ => None,
            }
        })
    {
        let p1name = param.name;
        let p2name = params[merge_index].name;
        tracing::info!("Merging together parameters {} and {}", p1name, p2name);

        // update the parameter's type
        match merge_info {
            MergeInfo::MakeSlice => take(&mut params[index].ty, |ty| match ty {
                Type::Ref(ty) => Type::Slice(ty),
                _ => unreachable!(),
            }),
            MergeInfo::MakeCallback { input_ptr_ty } => {
                take(&mut params[index].ty, |ty| match ty {
                    Type::Item(it) => match &*it {
                        Item::FunctionType {
                            name,
                            params,
                            return_type,
                        } => Type::Callback {
                            name: name.to_string(),
                            params: params.iter().map(|p| p.ty.clone()).collect(),
                            ret_ty: return_type.clone().map(Box::new),
                            input_ptr_ty,
                            fn_type: FnType::FnMut,
                        },
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                })
            }
        }

        // add the second parameter to the first's merge list
        params[index].merged.push(p2name);

        // delete the second parameter
        merged_params.push(merge_index);
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum Variation {
    NotUsed,
    Input,
    Output,
    InOut,
}

impl From<ParamFlags> for Variation {
    fn from(pf: ParamFlags) -> Self {
        match (pf.input(), pf.output()) {
            (false, false) => Variation::NotUsed,
            (true, false) => Variation::Input,
            (false, true) => Variation::Output,
            (true, true) => Variation::InOut,
        }
    }
}

impl Function {
    pub fn merge_params(&mut self, state: &mut State<'_>) -> Result<()> {
        let span = tracing::info_span!(
            "merge_params",
            function = %self.name
        );
        let _enter = span.enter();
        merge_params(&mut self.params, &mut self.merged_params, state)
    }

    fn unmerged_params(&self) -> impl DoubleEndedIterator<Item = (usize, &Param)> {
        self.params
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.merged_params.contains(i))
    }

    fn involves_void(&self, state: &mut State<'_>) -> Result<bool> {
        let ret_ty = self
            .return_type
            .as_ref()
            .map_or(Ok(false), |ret_ty| ret_ty.involves_void(state))?;
        any_res(self.params
            .iter()
            .map(|p| p.involves_void(state)).chain(Some(Ok(ret_ty))))
    }

    pub fn write(&self, state: &mut State<'_>) -> Result<()> {
        // comment it out if it involves void
        let involves_void = self.involves_void(state)?;
        if involves_void {
            state.begin_comment();
            swwriteln!(state, "Not generated due to containing a void type")?;
        }

        swwriteln!(state, "#[inline]")?;
        swwrite!(state, "pub fn {}", AsSnakeCase(self.name),)?;

        // write generics if we have to
        let mut need_closing_bracket = false;
        let generics = self
            .unmerged_params()
            .filter_map(|(_, p)| p.generics(state).transpose())
            .collect::<Result<Vec<_>>>()?;
        for (i, generic) in generics.into_iter().enumerate() {
            need_closing_bracket = true;

            let punct = if i == 0 { "<" } else { ", " };

            swwrite!(state, "{}{}", punct, generic)?;
        }

        if need_closing_bracket {
            swwrite!(state, ">")?;
        }

        swwrite!(state, "(")?;

        // write each input parameter
        for (i, param) in self.unmerged_params() {
            if matches!(param.variation, Variation::Output | Variation::NotUsed) {
                continue;
            }

            let mut ty = &param.ty;
            if matches!(param.variation, Variation::InOut) {
                ty = ty.unwrap_mut_ref()?;
            }

            let pp = ty.param_position(false, state)?;
            let is_optional = param.optional;
            let is_function = ty.is_function();
            let emit_option_tag = is_optional && !is_function;
            swwrite!(
                state,
                "{}: {}{}{}",
                FieldName(&param.name),
                if emit_option_tag { "Option<" } else { "" },
                pp,
                if emit_option_tag { ">" } else { "" },
            )?;

            if i != self.params.len() - 1 {
                swwrite!(state, ", ")?;
            }
        }

        swwrite!(state, ") -> ")?;

        // is this a fallible function?
        let is_fallible = match self.return_type {
            Some(ref rt) => rt.is_fallible(self.name, state),
            None => false,
        };

        if is_fallible {
            swwrite!(state, "Result<")?;
        }

        // write each output parameter
        let outputs = self
            .return_type
            .iter()
            .map(anyhow::Ok)
            .chain(self.unmerged_params().filter_map(|(_, param)| {
                if matches!(param.variation, Variation::Output | Variation::InOut) {
                    Some(param.ty.unwrap_mut_ref())
                } else {
                    None
                }
            }))
            .collect::<Result<Vec<_>>>()?;

        match outputs.len() {
            0 => swwrite!(state, "()")?,
            1 => {
                let rp = outputs[0].return_position(state)?;
                swwrite!(state, "{}", rp)?;
            }
            _ => {
                swwrite!(state, "(")?;
                for (i, output) in outputs.iter().enumerate() {
                    let rp = output.return_position(state)?;
                    swwrite!(state, "{}", rp,)?;

                    if i != outputs.len() - 1 {
                        swwrite!(state, ", ")?;
                    }
                }
                swwrite!(state, ")")?;
            }
        }

        if is_fallible {
            swwrite!(state, ">")?;
        }

        swwriteln!(state, " {{")?;

        // function body
        state.indent(|state| {
            // set up unused params
            // since they're unused we can just make them uninit
            for (_, param) in self
                .unmerged_params()
                .filter(|(_, p)| matches!(p.variation, Variation::NotUsed))
            {
                swwriteln!(
                    state,
                    "let {} = unsafe {{ mem::zeroed() }};",
                    param.name
                )?;
            }

            // set up output params
            // make them uninit so we can output into them
            for (_, param) in self
                .unmerged_params()
                .filter(|(_, p)| matches!(p.variation, Variation::Output))
            {
                param.setup_uninit_memory(&self.params, state)?;
            }

            // set up input/inout params
            for (_, param) in self
                .unmerged_params()
                .filter(|(_, p)| matches!(p.variation, Variation::Input | Variation::InOut))
            {
                param.setup_input(state)?;
            }

            // figure out the namespace to use
            let namespace = format_namespace(&self.namespace);

            // run the actual function
            swwrite!(
                state,
                "let return_value = unsafe {{ {}::{}(",
                namespace,
                self.name
            )?;
            for (i, param) in self.params.iter().enumerate() {
                if matches!(param.variation, Variation::InOut) {
                    swwrite!(state, "&mut {}", Sanitize(param.name))?;
                } else {
                    swwrite!(state, "{}", Sanitize(param.name))?;
                }
                if i != self.params.len() - 1 {
                    swwrite!(state, ", ")?;
                }
            }
            swwriteln!(state, ") }};")?;

            // if the return value is an error, return the last win32 error
            if is_fallible {
                swwriteln!(state, "if return_value == 0 {{")?;
                state.indent(|state| {
                    // return an error
                    swwriteln!(state, "return Err(crate::last_win32_error());")
                })?;
                swwriteln!(state, "}}")?;
            }

            // make returned finalize
            if let Some(ty) = self.return_type.as_ref() {
                let rty = ty.returnable_expr("return_value", &self.params, state)?;
                swwriteln!(
                    state,
                    "let real_return_value = {};",
                    rty 
                )?;
            }

            // parse returned parameters
            for (_, param) in self
                .unmerged_params()
                .rev()
                .filter(|(_, p)| matches!(p.variation, Variation::Output | Variation::InOut))
            {
                param.parse_returned_value(&self.params, state)?;
            }

            // return all of the output parameters
            match outputs.len() {
                0 => {
                    swwriteln!(state, "let _ = return_value;")?;
                    if is_fallible {
                        swwriteln!(state, "Ok(())")?;
                    }
                }
                1 => {
                    if is_fallible {
                        swwrite!(state, "Ok(")?;
                    }

                    swwrite!(
                        state,
                        "{}",
                        AsSnakeCase(
                            self.params
                                .iter()
                                .filter(|p| matches!(
                                    p.variation,
                                    Variation::InOut | Variation::Output
                                ))
                                .map(|p| p.name)
                                .next()
                                .unwrap_or("real_return_value")
                        )
                    )?;

                    if is_fallible {
                        swwriteln!(state, ")")?;
                    } else {
                        swwriteln!(state)?;
                    }
                }
                _ => {
                    if is_fallible {
                        swwrite!(state, "Ok(")?;
                    }

                    swwrite!(state, "(")?;

                    // return type firsties
                    if self.return_type.is_some() {
                        swwrite!(state, "real_return_value,")?;
                    }

                    // remainder of return values
                    let output_names = self
                        .unmerged_params()
                        .filter(|(_, p)| {
                            matches!(p.variation, Variation::InOut | Variation::Output)
                        })
                        .map(|(_, p)| p.name)
                        .collect::<Vec<_>>();
                    for (i, name) in output_names.iter().enumerate() {
                        swwrite!(state, "{}", name)?;
                        if i != output_names.len() - 1 {
                            swwrite!(state, ", ")?;
                        }
                    }

                    swwrite!(state, ")")?;

                    if is_fallible {
                        swwrite!(state, ")")?;
                    }

                    swwriteln!(state)?;
                }
            }

            anyhow::Ok(())
        })?;

        swwriteln!(state, "}}")?;

        if involves_void {
            state.end_comment();
        }

        Ok(())
    }
}

pub fn format_namespace(namespace: &str) -> String {
    namespace
        .replacen("Windows", "windows_sys", 1)
        .replace('.', "::")
}

struct Sanitize<T>(T);

impl<T: AsRef<str>> AsRef<str> for Sanitize<T> {
    fn as_ref(&self) -> &str {
        let s = self.0.as_ref();
        if s == "type" {
            "r#type"
        } else {
            s
        }
    }
}

impl<T: AsRef<str>> fmt::Display for Sanitize<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.as_ref();
        if s == "type" {
            f.write_str("r#type")
        } else if s == "use" {
            f.write_str("r#use")
        } else {
            f.write_str(s)
        }
    }
}

fn function_type(params: &[Param], ret_ty: Option<&Type>, state: &mut State<'_>) -> Result<String> {
    let mut expr = r#"Option<unsafe extern "system" fn("#.to_string();

    for (i, param) in params.iter().enumerate() {
        let pp = param.ty.win32_param_position(state)?;
        expr.push_str(&pp);

        if i != params.len() - 1 {
            expr.push_str(", ");
        }
    }

    expr.push(')');

    if let Some(ret_ty) = ret_ty {
        let pp = ret_ty.win32_param_position(state)?;
        write!(expr, " -> {}", pp,).ok();
    }

    expr.push('>');

    Ok(expr)
}

struct FieldName<T>(T);

impl<T: AsRef<str>> AsRef<str> for FieldName<T> {
    fn as_ref(&self) -> &str {
        let s = self.0.as_ref();
        if s == "type" {
            "r#type"
        } else {
            s
        }
    }
}

impl<T: AsRef<str>> fmt::Display for FieldName<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.as_ref();
        if s == "type" {
            f.write_str("r#type")
        } else if s == "use" {
            f.write_str("r#use")
        } else {
            fmt::Display::fmt(&AsSnakeCase(s), f)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FnType {
    FnMut,
}

fn likely_slice_count(name: &str) -> bool {
    name.contains("max")
        || name.starts_with('c')
        || name.starts_with('n')
        || name.contains("cch")
        || name.ends_with("Size")
        || name.ends_with("size")
        || name.ends_with("Count")
        || name.ends_with("count")
}

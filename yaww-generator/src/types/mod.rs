// MIT/Apache2 License

use crate::{swwriteln, State};
use anyhow::{anyhow, Context, Result};
use heck::{
    AsLowerCamelCase, AsShoutySnakeCase, AsSnakeCase, AsUpperCamelCase, ToSnakeCase,
    ToUpperCamelCase,
};
use once_cell::unsync::OnceCell;
use std::{
    borrow::Cow,
    cell::RefCell,
    fmt::{self, Write},
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};
use windows_metadata::reader::{self, ParamFlags};

mod ty;
pub use ty::Type;

// TODO: setup callback for returnable_expr()

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    /// An item we already import from `windows-rs`.
    AlreadyImported(&'static str),
    /// A handle to a foreign resource.
    ///
    /// Implied not to be thread safe.
    ForeignHandle(&'static str),
    /// A function type with the given types as arguments.
    FunctionType {
        name: &'static str,
        params: Vec<Param>,
        return_type: Option<Type>,
    },
    /// Special type.
    Special(Special),
    /// A struct type.
    Structure {
        name: Cow<'static, str>,
        is_union: bool,
        fields: Vec<Field>,
        namespace: String,
        packing: Option<usize>,
    },
    /// A set of constants.
    ConstantSet {
        name: &'static str,
        constants: Vec<Constant>,
        ty: Type,
    },
    /// Interface item, desist immediately.
    Interface,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Constant {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    pub name: Cow<'static, str>,
    pub ty: ResolveLater,
}

impl Field {
    pub fn ty(&self, state: &mut State<'_>) -> Result<&Type> {
        self.ty
            .resolved
            .get_or_try_init(|| state.get_item(&*self.ty.unresolved.as_ref().unwrap().0))
            .with_context(|| format!("Resolving field {}", self.name))
    }
}

#[derive(Debug)]
pub struct ResolveLater {
    unresolved: Option<crate::DebugType<'static>>,
    resolved: OnceCell<Type>,
}

// these are 100% broken but I don't care

impl PartialEq for ResolveLater {
    fn eq(&self, other: &Self) -> bool {
        self.resolved == other.resolved
    }
}

impl Eq for ResolveLater {}

impl ResolveLater {
    pub fn resolved(ty: Type) -> Self {
        ResolveLater {
            unresolved: None,
            resolved: OnceCell::with_value(ty),
        }
    }

    pub fn unresolved(ty: impl Into<crate::DebugType<'static>>) -> Self {
        ResolveLater {
            unresolved: Some(ty.into()),
            resolved: OnceCell::new(),
        }
    }
}

fn struct_impl_block(
    name: &str,
    fields: &[Field],
    is_union: bool,
    namespace: &str,
    state: &mut State<'_>,
) -> Result<()> {
    // whether or not all fields are thin
    let is_thin = fields
        .iter()
        .map(|f| f.ty(state)?.thin(state))
        .try_fold(true, |acc, b| anyhow::Ok(acc && b?))?;

    let display_name = name.to_upper_camel_case();
    let namespace = format_namespace(namespace);
    let win32_name = format!("{}::{}", namespace, name);

    swwriteln!(state, "impl {} {{", &display_name)?;
    state.indent(|state| {
        // method to convert to win32
        swwriteln!(state, "fn to_win32(&self) -> {} {{", &win32_name)?;
        state.indent(|state| {
            // if this is a thin struct, just transmute
            if is_thin {
                swwriteln!(state, "// SAFETY: this is a thin struct")?;
                swwriteln!(state, "unsafe {{ std::mem::transmute_copy(self) }}")
            } else {
                // deconstruct this struct
                swwrite!(state, "let Self {{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    swwrite!(state, "{}", Sanitize(&field.name))?;
                    if i != fields.len() - 1 {
                        swwrite!(state, ", ")?;
                    }
                }
                swwriteln!(state, " }} = self;")?;

                // begin converting fields to other fields
                for field in fields {
                    let fname = FieldName(&field.name).to_string();
                    let rname = &field.name;
                    let expr = field.ty(state)?.to_win32(&fname, state)?;
                    swwriteln!(state, "let {} = {};", Sanitize(rname), expr)?;
                }

                // construct the new edition
                swwrite!(state, "{} {{ ", win32_name)?;
                for (i, field) in fields.iter().enumerate() {
                    swwrite!(state, "{}", Sanitize(&field.name))?;
                    if i != fields.len() - 1 {
                        swwrite!(state, ", ")?;
                    }
                }
                swwriteln!(state, " }}")
            }
        })?;
        swwriteln!(state, "}}")?;

        // method to convert from win32
        swwriteln!(
            state,
            "unsafe fn from_win32(win32: {}) -> Self {{",
            &win32_name
        )?;
        state.indent(|state| {
            if is_thin {
                swwriteln!(state, "// SAFETY: this is a thin struct")?;
                swwriteln!(state, "std::mem::transmute(win32)")
            } else {
                // deconstruct the struct
                swwrite!(state, "let {} {{ ", &win32_name)?;
                for (i, field) in fields.iter().enumerate() {
                    swwrite!(state, "{}", Sanitize(&field.name))?;
                    if i != fields.len() - 1 {
                        swwrite!(state, ", ")?;
                    }
                }
                swwriteln!(state, " }} = win32;")?;

                // begin converting fields to other fields
                for field in fields {
                    let fname = Sanitize(&field.name).to_string();
                    let rname = FieldName(&field.name).to_string();
                    let expr = field.ty(state)?.from_win32(&fname, state)?;
                    swwriteln!(state, "let {} = {};", Sanitize(rname), expr)?;
                }

                // construct the new edition
                swwrite!(state, "Self {{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    swwrite!(state, "{}", FieldName(&field.name))?;
                    if i != fields.len() - 1 {
                        swwrite!(state, ", ")?;
                    }
                }
                swwriteln!(state, " }}")
            }
        })?;
        swwriteln!(state, "}}")?;

        anyhow::Ok(())
    })?;
    swwriteln!(state, "}}")?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum Special {
    Lparam,
    Wparam,
}

impl Item {
    pub fn name(&self) -> Cow<'static, str> {
        match self {
            Self::AlreadyImported(name) | Self::ForeignHandle(name) => (*name).into(),
            Self::FunctionType { name, .. } => (*name).into(),
            Self::ConstantSet { name, .. } => (*name).into(),
            Self::Structure { name, .. } => name.clone(),
            Self::Interface => "interface".into(),
            Self::Special(Special::Wparam) => "Wparam".into(),
            Self::Special(Special::Lparam) => "Lparam".into(),
        }
    }

    pub fn write(&self, state: &mut State<'_>) -> Result<()> {
        match self {
            Self::AlreadyImported(..)
            | Self::FunctionType { .. }
            | Self::Special(..)
            | Self::Interface => { /* do nothing */ }
            Self::Structure {
                name,
                is_union,
                fields,
                namespace,
                packing,
            } => {
                // zsts dont exist in win32
                if fields.is_empty() {
                    return Ok(());
                }

                let is_union = *is_union;
                // don't write if we contain an interface
                if fields.iter().try_fold(false, |acc, f| {
                    anyhow::Ok(acc || f.ty(state)?.is_interface())
                })? {
                    return Ok(());
                }

                if !is_union {
                    swwriteln!(
                        state,
                        "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]"
                    )?;
                }

                swwrite!(state, "#[repr(C")?;
                if let Some(packing) = packing {
                    swwrite!(state, ", packed({})", packing)?;
                }
                swwriteln!(state, ")]")?;

                swwrite!(state, "pub ")?;
                if is_union {
                    swwrite!(state, "union ")?;
                } else {
                    swwrite!(state, "struct ")?;
                }
                swwrite!(state, "{}", AsUpperCamelCase(name))?;

                if fields.iter().try_fold(false, |acc, f| {
                    anyhow::Ok(acc || f.ty(state)?.needs_lifetime(state)?)
                })? {
                    swwrite!(state, "<'a>")?;
                }
                swwriteln!(state, " {{")?;

                // write all of the fields
                state.indent(|state| {
                    for field in fields {
                        let ty = field.ty(state)?;
                        let fp = ty.field_position(state)?;
                        swwriteln!(state, "pub {}: {},", FieldName(&field.name), fp)?;
                    }

                    anyhow::Ok(())
                })?;

                swwriteln!(state, "}}")?;

                // write the impl block
                struct_impl_block(name, fields, is_union, namespace, state)?;
            }
            Self::ConstantSet {
                name,
                constants,
                ty,
            } => {
                for constant in constants {
                    // write the constant line
                    swwriteln!(
                        state,
                        "pub const {}: {} = {};",
                        AsShoutySnakeCase(constant.name),
                        ty.const_position()?,
                        constant.value
                    )?;
                }
            }
            Self::ForeignHandle(name) => {
                swwriteln!(
                    state,
                    r#"
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct {0} {{
    handle: isize, 
    _thread_unsafe: PhantomData<*mut ()>,
}}

impl {0} {{
    pub const fn null() -> Self {{
        // SAFETY: null is always a valid pointer
        unsafe {{ Self::new(0) }}
    }}

    pub const unsafe fn new(handle: isize) -> Self {{
        Self {{
            handle,
            _thread_unsafe: PhantomData,
        }}
    }}

    pub fn into_raw(self) -> isize {{
        self.handle
    }}
}}

impl fmt::Debug for {0} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        fmt::UpperHex::fmt(&self.handle, f)
    }}
}}

#[cfg(feature = "breadthread")]
unsafe impl breadthread::Compatible for {0} {{
    fn representative(&self) -> usize {{
        self.into_raw() as usize
    }}
}}
                    "#,
                    AsUpperCamelCase(name),
                )?;

                ywriteln!(
                    state,
                    "
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct {0} <Tag = YawwTag> {{
    inner: Object<safer_wingui::{0}, Tag>,
}}

impl<Tag> {0}<Tag> {{
    pub(crate) fn new(inner: Object<safer_wingui::{0}, Tag>) -> Self {{
        Self {{
            inner,
        }}
    }}
}}
                    ",
                    AsUpperCamelCase(name),
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: &'static str,
    pub return_type: Option<Type>,
    pub params: Vec<Param>,
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
                    "let mut {} = mem::MaybeUninit::uninit();",
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
            Type::Callback { fn_type, .. } => {
                // the actual function takes a pointer to the inner
                // function and casts it to call it
                let ptr_name = self.merged.first().unwrap();
                match fn_type {
                    FnType::FnMut => swwriteln!(
                        state,
                        "let {} = (&mut {}).as_ptr() as *mut _ as _;",
                        Sanitize(ptr_name),
                        FieldName(self.name),
                    )?,
                }
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
                    .filter(|(_, p)| *p != &**input_ptr_ty)
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
    params: &mut Vec<Param>,
    state: &mut State<'_>,
) -> Result<()> {
    // try to find a param to be merged
    while let Some((index, param, merge_index)) =
        params.iter().enumerate().rev().find_map(|(_, param)| {
            match &param.ty {
                _ => None,
            }
        }) {
        
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
    pub fn write(&self, state: &mut State<'_>) -> Result<()> {
        swwriteln!(state, "#[inline]")?;
        swwrite!(state, "pub fn {}", AsSnakeCase(self.name),)?;

        // write generics if we have to
        let mut need_closing_bracket = false;
        let generics = self
            .params
            .iter()
            .filter_map(|p| p.generics(state).transpose())
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
        for (i, param) in self.params.iter().enumerate() {
            if matches!(param.variation, Variation::Output | Variation::NotUsed) {
                continue;
            }

            let mut ty = &param.ty;
            if matches!(param.variation, Variation::InOut) {
                ty = ty.unwrap_mut_ref()?;
            }

            let pp = ty.param_position(false, state)?;
            let is_optional = param.optional;
            swwrite!(
                state,
                "{}: {}{}{}",
                FieldName(&param.name),
                if is_optional { "Option<" } else { "" },
                pp,
                if is_optional { ">" } else { "" },
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
            .chain(self.params.iter().filter_map(|param| {
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
        // TODO
        state.indent(|state| {
            // set up unused params
            // since they're unused we can just make them uninit
            for param in self
                .params
                .iter()
                .filter(|p| matches!(p.variation, Variation::NotUsed))
            {
                swwriteln!(
                    state,
                    "let {} = unsafe {{ mem::uninitialized() }};",
                    param.name
                )?;
            }

            // set up output params
            // make them uninit so we can output into them
            for param in self
                .params
                .iter()
                .filter(|p| matches!(p.variation, Variation::Output))
            {
                param.setup_uninit_memory(&self.params, state)?;
            }

            // set up input/inout params
            for param in self
                .params
                .iter()
                .filter(|p| matches!(p.variation, Variation::Input | Variation::InOut))
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

            // parse returned parameters
            for param in self
                .params
                .iter()
                .rev()
                .filter(|p| matches!(p.variation, Variation::Output | Variation::InOut))
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
                                .unwrap_or("return_value")
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
                        swwrite!(state, "return_value,")?;
                    }

                    // remainder of return values
                    let output_names = self
                        .params
                        .iter()
                        .filter(|p| matches!(p.variation, Variation::InOut | Variation::Output))
                        .map(|p| p.name)
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

#[derive(Debug, PartialEq, Eq)]
pub enum FnType {
    FnMut,
}

fn likely_slice_count(name: &str) -> bool {
    name.starts_with('n')
        || name.contains("max")
        || name.starts_with('c')
        || name.contains("cch")
        || name.ends_with("size")
        || name.ends_with("count")
}

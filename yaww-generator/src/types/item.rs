// MIT/Apache2 License

use super::{format_namespace, Constant, Field, FieldName, Param, Sanitize, Special, Type};
use crate::State;
use anyhow::{anyhow, Result};
use heck::{
    AsLowerCamelCase, AsShoutySnakeCase, AsSnakeCase, AsUpperCamelCase, ToSnakeCase,
    ToUpperCamelCase,
};
use std::borrow::Cow;

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

fn struct_varsize(fields: &[Field], state: &mut State<'_>) -> Result<bool> {
    fields.iter().map(|field| {
        let ty = field.ty(state)?;
        anyhow::Ok(ty.involves_varsized())
    }).try_fold(false, |acc, b| Ok(acc || b?))
} 

fn struct_impl_block(
    name: &str,
    fields: &[Field],
    is_union: bool,
    namespace: &str,
    state: &mut State<'_>,
) -> Result<()> {
    // whether or not all fields are thin
    let mut is_thin = fields
        .iter()
        .map(|f| f.ty(state)?.thin(state))
        .try_fold(true, |acc, b| anyhow::Ok(acc && b?))?;
    let is_varsize = struct_varsize(fields, state)?;

    // we aren't this if the last field is a size
    let first_field_name = &fields.first().unwrap().name;
    if first_field_name.ends_with("size") || first_field_name.ends_with("Size") {
        is_thin = false;
    }

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
                swwriteln!(state, "unsafe {{ mem::transmute_copy(self) }}")
            } else {
                // deconstruct this struct
                swwrite!(state, "let Self {{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    swwrite!(state, "{}", FieldName(&field.name))?;
                    if i != fields.len() - 1 {
                        swwrite!(state, ", ")?;
                    }
                }
                swwriteln!(state, " }} = self;")?;

                // begin converting fields to other fields
                for (i, field) in fields.iter().enumerate() {
                    let fname = FieldName(&field.name).to_string();
                    let rname = &field.name;

                    if i == 0 && (field.name.ends_with("Size")) {
                        // field needs to be set to the size of the new structure
                        swwriteln!(
                            state,
                            "let {} = mem::size_of::<{}>() as _;",
                            Sanitize(rname),
                            &win32_name
                        )?;
                    } else {
                        let expr = field.ty(state)?.to_win32(&fname, state)?;
                        swwriteln!(state, "let {} = {};", Sanitize(rname), expr)?;
                    }
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
                swwriteln!(state, "mem::transmute(win32)")
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

                let involves_void = fields
                    .iter()
                    .map(|f| f.involves_void(state))
                    .try_fold(false, |acc, b| anyhow::Ok(acc || b?))?;
                if involves_void {
                    state.begin_comment();
                    swwriteln!(state, "Not generated due to containing a void type")?;
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

                if involves_void {
                    state.end_comment();
                }
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
    handle: NonZeroIsize, 
    _thread_unsafe: PhantomData<*mut ()>,
}}

impl {0} {{
    pub const unsafe fn new(handle: isize) -> Self {{
        Self {{
            handle: NonZeroIsize::new_unchecked(handle),
            _thread_unsafe: PhantomData,
        }}
    }}

    pub const unsafe fn new_optional(handle: isize) -> Self {{
        NonZeroIsize::new(handle).map(|handle| {{
            Self {{
                handle,
                _thread_unsafe: PhantomData,
            }}
        }})
    }}

    pub fn into_raw(self) -> isize {{
        self.handle.get()
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

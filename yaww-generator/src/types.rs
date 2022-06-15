// MIT/Apache2 License

use crate::{swwriteln, State};
use anyhow::{anyhow, Context, Result};
use heck::{AsShoutySnakeCase, AsUpperCamelCase};
use once_cell::unsync::OnceCell;
use std::{borrow::Cow, cell::RefCell, fmt, rc::Rc, sync::Once};
use windows_metadata::reader::{self, ParamFlags};

#[derive(Debug)]
pub enum Type {
    Void,
    /// A primitive type, like `u32` or `bool`.
    Primitive(&'static str),
    /// Something like a string.
    String,
    /// Something like an `OsString`.
    OsString,
    /// Mutable reference to another type.
    MutRef(Box<Type>),
    /// Immutable reference to another type.
    Ref(Box<Type>),
    /// An item we've defined.
    Item(Rc<Item>),
    /// An array of some type.
    Array(Box<Type>, usize),
}

impl Type {
    /// In the constant position.
    fn const_position(&self) -> Result<&'static str> {
        match self {
            Self::Primitive(s) => Ok(s),
            Self::String => Ok("&str"),
            Self::Item(i) => match &**i {
                Item::AlreadyImported(s) => Ok(s),
                Item::ForeignHandle(s) => Ok(s),
                item => Err(anyhow!("item invalid in const position: {:?}", item)),
            },
            ty => Err(anyhow!("type invalid in const position: {:?}", ty)),
        }
    }

    /// Whether or not we need a lifetime in the field position.
    fn needs_lifetime(&self, state: &mut State<'_>) -> Result<bool> {
        Ok(match self {
            Self::Ref(_) | Self::MutRef(_) | Self::String | Self::OsString => true,
            Self::Item(item) => match &**item {
                Item::Structure { fields, .. } => fields.iter().try_fold(false, |acc, f| {
                    anyhow::Ok(acc || f.ty(state)?.needs_lifetime(state)?)
                })?,
                _ => false,
            },
            _ => false,
        })
    }

    fn is_interface(&self) -> bool {
        match self {
            Self::Item(item) => matches!(&**item, Item::Interface),
            _ => false,
        }
    }

    /// The type in the field position.
    fn field_position(&self, state: &mut State<'_>) -> Result<String> {
        Ok(match self {
            Self::Primitive(s) => s.to_string(),
            Self::String => "Cow<'a, str>".to_string(),
            Self::OsString => "Cow<'a, OsStr>".to_string(),
            Self::Ref(ty) => format!("&'a {}", ty.field_position(state)?),
            Self::MutRef(ty) => format!("&'a mut {}", ty.field_position(state)?),
            Self::Item(item) => match &**item {
                Item::AlreadyImported(name) | Item::ForeignHandle(name) => name.to_string(),
                Item::Structure { name, .. } => {
                    if self.needs_lifetime(state)? {
                        format!("{}<'a>", name)
                    } else {
                        name.to_string()
                    }
                }
                Item::ConstantSet { ty, .. } => ty.field_position(state)?,
                Item::FunctionType { .. } => "todo_fn".to_string(),
                Item::Special(Special::Wparam) => "Wparam".to_string(),
                Item::Special(Special::Lparam) => "Lparam".to_string(),
                Item::Interface => Err(anyhow!("interface invalid in field position"))?,
            },
            Self::Array(ty, size) => format!("[{}; {}]", ty.field_position(state)?, size),
            Self::Void => "todo_void".to_string(),
        })
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Constant {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Number(i128),
    String(String),
    Boolean(bool),
    Float(f64),
}

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

#[derive(Debug)]
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
            } => {
                let is_union = *is_union;
                // don't write if we contain an interface
                if fields.iter().try_fold(false, |acc, f| {
                    anyhow::Ok(acc || f.ty(state)?.is_interface())
                })? {
                    return Ok(());
                }

                if is_union {
                    swwriteln!(
                        state,
                        "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]"
                    )?;
                }
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
                        swwriteln!(state, "pub {}: {},", field.name, fp)?;
                    }

                    anyhow::Ok(())
                })?;

                swwriteln!(state, "}}")?;
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
                    "
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
                    ",
                    AsUpperCamelCase(name),
                )?;

                ywriteln!(
                    state,
                    "
#[repr(transparent)]
#[derive(Copy, Clone)]
struct {0}Wrapper(safer_wingui::{0});

impl From<safer_wingui::{0}> for {0}Wrapper {{
    fn from(handle: safer_wingui::{0}) -> Self {{
        Self(handle)
    }}
}}

impl From<{0}Wrapper> for safer_wingui::{0} {{
    fn from(wrapper: {0}Wrapper) -> Self {{
        wrapper.0
    }}
}} 

unsafe impl Compatible for {0}Wrapper {{
    fn representative(&self) -> usize {{
        self.into_raw() as usize
    }}
}}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct {0} <Tag = YawwTag> {{
    inner: Object<{0}Wrapper, Tag>,
}}

impl<Tag> {0}<Tag> {{
    pub(crate) fn new(inner: Object<{0}Wrapper, Tag>) -> Self {{
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
}

#[derive(Debug)]
pub struct Param {
    pub name: &'static str,
    pub ty: Type,
    pub variation: Variation,
    pub optional: bool,
}

#[derive(Debug)]
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
        swwriteln!(state, "// TODO: {}", self.name)?;
        Ok(())
    }
}

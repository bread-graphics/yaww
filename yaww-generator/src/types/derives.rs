// BSL 1.0 License

use anyhow::Result;
use crate::{State, Item};
use std::{ops::Mul, iter::Product};

use super::Type;

/// The derives that a type can have.
#[derive(Copy, Clone)]
pub struct Derives {
    pub copy: bool,
    pub clone: bool,
    pub floating: bool,
    pub default: bool,
}

impl Default for Derives {
    fn default() -> Self {
        Self { copy: true, clone: true, floating: false, default: true }
    }
}

impl Derives {
    pub fn emit(self, state: &mut State<'_>) -> Result<()> {
        // by default we can emit these no problem
        swwrite!(state, "#[derive(Debug, PartialEq, Default")?;

        // if we implement copy, we can add Copy
        if self.copy {
            swwrite!(state, ", Copy")?;
        }

        if self.clone {
            swwrite!(state, ", Clone")?;
        }

        // if we don't use any floats, we can add comparisons
        if !self.floating {
            swwrite!(state, ", Eq, PartialOrd, Ord, Hash")?;
        }

        swwriteln!(state, ")]")?;
        Ok(())
    }

    pub fn of(ty: &Type, state: &mut State<'_>) -> Result<Self> {
        Ok(match ty {
            Type::Primitive(name) => Self {
                copy: true,
                clone: true,
                floating: matches!(&**name, "f32" | "f64"),
                default: true,
            },
            ty if ty.is_os_str_ref() || ty.is_str_ref() => Self {
                copy: false,
                clone: true,
                floating: false,
                default: true,
            },
            Type::MutRef(ty) => Self {
                copy: false,
                clone: false,
                default: false,
                ..Self::of(ty, state)?
            },
            Type::Ref(ty) => Self {
                copy: false,
                clone: true,
                ..Self::of(ty, state)?
            },
            Type::Array(ty, sz) => {
                let mut d = Self::of(ty, state)?;
                if *sz > 32 {
                    d.default = false;
                }
                d
            },
            Type::Item(item) => match &**item {
                Item::AlreadyImported(_) => Self::default(),
                Item::ConstantSet { ty, .. } => Self::of(ty, state)?,
                Item::ForeignHandle(_) => Self::default(),
                Item::Special(_) => Self::default(),
                Item::Structure { fields, .. } => {
                    fields.iter().map(|field| {
                        let ty = field.ty(state)?;
                        Self::of(ty, state)
                    }).try_fold(
                        Self::default(),
                        |acc, x| {
                            anyhow::Ok(acc * x?)
                        }
                    )?
                } 
                Item::FunctionType { .. } => {
                    Derives {
                        copy: true,
                        clone: true,
                        floating: true,
                        default: true,
                    }
                }
                item => panic!("unsupported item: {:?}", item), 
            }
            Type::Void | Type::Stub => {
                // todo?
                Derives::default()
            } 
            field => panic!("unsupported field type: {:?}", field),
        })
    }
}

impl Mul for Derives {
    type Output = Derives;
    
    fn mul(self, other: Self) -> Self::Output {
        Self {
            copy: self.copy && other.copy,
            clone: self.clone && other.clone,
            floating: self.floating || other.floating,
            default: self.default && other.default,
        }
    }
}

impl Product for Derives {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Derives::default(), |acc, x| acc * x)
    }
}
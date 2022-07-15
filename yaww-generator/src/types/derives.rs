//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use crate::{Item, State};
use anyhow::Result;
use std::{iter::Product, ops::Mul};

use super::Type;

/// The derives that a type can have.
#[derive(Copy, Clone)]
pub struct Derives {
    pub debug: bool,
    pub copy: bool,
    pub clone: bool,
    pub partial_eq: bool,
    pub eq: bool,
    pub partial_ord: bool,
    pub ord: bool,
    pub hash: bool,
    pub default: bool,
}

impl Default for Derives {
    fn default() -> Self {
        Self {
            debug: true,
            copy: true,
            clone: true,
            partial_eq: true,
            eq: true,
            partial_ord: true,
            ord: true,
            hash: true,
            default: true,
        }
    }
}

impl Derives {
    pub fn emit(self, state: &mut State<'_>) -> Result<()> {
        let mut emitted_any = false;

        let mut emit_derive = |cond: bool, name: &str| {
            if cond {
                // emit either the derive or a comma
                if emitted_any {
                    swwrite!(state, ", ")?;
                } else {
                    swwrite!(state, "#[derive(")?;
                    emitted_any = true;
                }

                swwrite!(state, "{}", name)?;
            }

            anyhow::Ok(())
        };

        emit_derive(self.debug, "Debug")?;
        emit_derive(self.copy, "Copy")?;
        emit_derive(self.clone, "Clone")?;
        emit_derive(self.partial_eq, "PartialEq")?;
        emit_derive(self.eq, "Eq")?;
        emit_derive(self.partial_ord, "PartialOrd")?;
        emit_derive(self.ord, "Ord")?;
        emit_derive(self.hash, "Hash")?;
        emit_derive(self.default, "Default")?;

        if emitted_any {
            swwriteln!(state, ")]")?;
        }

        Ok(())
    }

    pub fn of(ty: &Type, state: &mut State<'_>) -> Result<Self> {
        Ok(match ty {
            Type::Primitive(name) => {
                let is_floating = matches!(&**name, "f32" | "f64");
                let is_guid = matches!(&**name, "GUID");
                let has_eq_ord_or_hash = !is_floating && !is_guid;
                Self {
                    debug: !is_guid,
                    partial_eq: !is_guid,
                    default: !is_guid,
                    eq: has_eq_ord_or_hash,
                    partial_ord: has_eq_ord_or_hash,
                    ord: has_eq_ord_or_hash,
                    hash: has_eq_ord_or_hash,
                    ..Default::default()
                }
            }
            ty if ty.is_os_str_ref() || ty.is_str_ref() => Self {
                copy: false,
                ..Default::default()
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
            }
            Type::Item(item) => match &**item {
                Item::AlreadyImported(_) => Self::default(),
                Item::ConstantSet { ty, .. } => Self::of(ty, state)?,
                Item::ForeignHandle(_) => Self::default(),
                Item::Special(_) => Self::default(),
                Item::Structure { fields, .. } => fields
                    .iter()
                    .map(|field| {
                        let ty = field.ty(state)?;
                        Self::of(ty, state)
                    })
                    .try_fold(Self::default(), |acc, x| anyhow::Ok(acc * x?))?,
                Item::FunctionType { .. } => Derives {
                    debug: false,
                    partial_eq: false,
                    partial_ord: false,
                    eq: false,
                    ord: false,
                    hash: false,
                    ..Default::default()
                },
                item => panic!("unsupported item: {:?}", item),
            },
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
            debug: self.debug && other.debug,
            copy: self.copy && other.copy,
            clone: self.clone && other.clone,
            partial_eq: self.partial_eq && other.partial_eq,
            eq: self.eq && other.eq,
            partial_ord: self.partial_ord && other.partial_ord,
            ord: self.ord && other.ord,
            hash: self.hash && other.hash,
            default: self.default && other.default,
        }
    }
}

impl Product for Derives {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Derives::default(), |acc, x| acc * x)
    }
}

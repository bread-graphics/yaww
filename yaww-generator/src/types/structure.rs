//                Copyright John Nunley 2022
// Distributed under the Boost Software License, Version 1.0.
//        (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use super::Type;
use crate::State;
use anyhow::{Context, Result};
use once_cell::unsync::OnceCell;
use std::borrow::Cow;

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

    pub fn involves_void(&self, state: &mut State<'_>) -> Result<bool> {
        let ty = self.ty(state)?;
        ty.involves_void(state)
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

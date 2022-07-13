// MIT/Apache2 License

use super::{function_type, likely_slice_count, FnType, Item, Param, Special, Variation, any_res, all_res};
use crate::{State, format_namespace};
use anyhow::{anyhow, Result};
use heck::{
    AsLowerCamelCase, AsShoutySnakeCase, AsSnakeCase, AsUpperCamelCase, ToSnakeCase,
    ToUpperCamelCase,
};
use std::{
    borrow::Cow,
    fmt::Write,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Void,
    /// Stubbed out to be implemented in the crate body.
    Stub,
    /// A primitive type, like `u32` or `bool`.
    Primitive(Cow<'static , str>),
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
    /// A slice of values.
    Slice(Box<Type>),
    /// A callback value.
    Callback {
        name: String,
        params: Vec<Type>,
        ret_ty: Option<Box<Type>>,
        input_ptr_ty: PtrClass,
        fn_type: FnType,
    },
    /// A variable-size value containing this inner type.
    /// 
    /// This is only valid in structures.
    VariableSize(Box<Type>),
}

impl Type {
    pub fn primitive(name: &'static str) -> Self {
        Self::Primitive(Cow::Borrowed(name))
    }

    pub fn is_str_ref(&self) -> bool {
        match self {
            Self::String => true,
            Self::Ref(s) | Self::MutRef(s) => s.is_str_ref(),
            _ => false,
        }
    }

    pub fn is_os_str_ref(&self) -> bool {
        match self {
            Self::OsString => true,
            Self::Ref(s) | Self::MutRef(s) => s.is_os_str_ref(),
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Self::Item(item) => matches!(&**item, Item::FunctionType { .. }),
            _ => false, 
        }
    }

    fn any_match_in_tree<F: FnMut(&Self) -> bool>(
        &self,
        state: &mut State<'_>,
        mut tester: F
    ) -> Result<bool> {
        if tester(self) {
            return Ok(true);
        }

        match self {
            Self::Ref(ty) | Self::MutRef(ty) | Self::Slice(ty) | Self::Array(ty, _) => {
                ty.any_match_in_tree(state, tester)
            }
            Self::Callback { params, ret_ty, .. } => {
                // prevent more than one level of type recursion
                let mut dyn_tester: &mut dyn FnMut(&Self) -> bool = &mut tester;
                let rt = ret_ty
                    .as_ref()
                    .map_or(Ok(false), |ret_ty| ret_ty.any_match_in_tree(state, &mut dyn_tester))?;
                any_res(params
                    .iter()
                    .map(|ty| ty.any_match_in_tree(state, &mut dyn_tester)).chain(Some(Ok(rt))))
            }
            Self::Item(item) => {
                if let Item::Structure { fields, .. } = &**item {
                    let mut dyn_tester: &mut dyn FnMut(&Self) -> bool = &mut tester;
                    any_res(fields
                        .iter()
                        .map(|field| field.ty(state)?.any_match_in_tree(state, &mut dyn_tester)))
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }

    pub fn involves_stub(&self, state: &mut State<'_>) -> Result<bool> {
        self.any_match_in_tree(state, |t| matches!(t, Self::Stub))
    }

    pub fn involves_void(&self, state: &mut State<'_>) -> Result<bool> {
        self.any_match_in_tree(state, |t| matches!(t, Self::Void))
    }

    /// In the constant position.
    pub fn const_position(&self) -> Result<&str> {
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

    pub fn generics(&self, state: &mut State<'_>) -> Result<Option<String>> {
         Ok(match &self {
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

    pub fn unwrap_mut_ref(&self) -> Result<&Type> {
        match self {
            Self::MutRef(ty) => Ok(&**ty),
            ty => Ok(ty),
        }
    }

    /// Whether or not we need a lifetime in the field position.
    pub fn needs_lifetime(&self, state: &mut State<'_>) -> Result<bool> {
        Ok(match self {
            Self::Ref(_) | Self::MutRef(_) | Self::String | Self::OsString | Self::Slice(_) => true,
            Self::Item(item) => match &**item {
                Item::Structure { fields, .. } => any_res(fields.iter().map(|f| f.ty(state)?.needs_lifetime(state)))?,
                _ => false,
            },
            _ => false,
        })
    }

    pub fn is_interface(&self) -> bool {
        match self {
            Self::Item(item) => matches!(&**item, Item::Interface),
            Self::Ref(ty) | Self::MutRef(ty) => ty.is_interface(),
            _ => false,
        }
    }

    /// The type in the field position.
    pub fn field_position(&self, state: &mut State<'_>) -> Result<String> {
        Ok(match self {
            Self::Primitive(s) => s.to_string(),
            ty if ty.is_str_ref() => "Cow<'a, CStr>".to_string(),
            ty if ty.is_os_str_ref() => "Cow<'a, [u16]>".to_string(),
            Self::Ref(ty) => format!("Cow<'a, {}>", ty.field_position(state)?),
            Self::MutRef(ty) => format!("&'a mut {}", ty.field_position(state)?),
            Self::Slice(ty) => format!("Cow<'a, [{}]>", ty.field_position(state)?),
            Self::Item(item) => match &**item {
                Item::AlreadyImported(name) => name.to_string(),
                Item::ForeignHandle(name) => format!("Option<{}>", AsUpperCamelCase(name)),
                Item::Structure { name, .. } => {
                    if self.needs_lifetime(state)? {
                        format!("{}<'a>", AsUpperCamelCase(name))
                    } else {
                        name.to_upper_camel_case()
                    }
                }
                Item::ConstantSet { ty, .. } => ty.field_position(state)?,
                Item::FunctionType {
                    params,
                    return_type,
                    ..
                } => function_type(params, return_type.as_ref(), state)?,
                Item::Special(s) => s.name().to_string(),
                Item::Interface => return Err(anyhow!("interface invalid in field position")),
            },
            Self::Array(ty, size) => format!("[{}; {}]", ty.field_position(state)?, size),
            Self::Void => "todo_void".to_string(),
            Self::Callback { .. } => return Err(anyhow!("callback invalid in field position")),
            Self::VariableSize(ty) => {
                // this turns this structure into a real variable-size type
                format!("[{}]", ty.field_position(state)?)
            }
            Self::Stub => "STUBBED".to_string(),
            _ => unreachable!(),
        })
    }

    /// The type in the parameter position.
    pub fn param_position(&self, is_cb: bool, state: &mut State<'_>) -> Result<String> {
        Ok(match self {
            Self::Primitive(s) => s.to_string(),
            Self::String => "&CStr".to_string(),
            Self::OsString => "&OsStr".to_string(),
            Self::Ref(ty) if matches!(&**ty, Self::String) => "&CStr".to_string(),
            Self::Ref(ty) if matches!(&**ty, Self::OsString) => "&OsStr".to_string(),
            Self::Ref(ty) => format!("&{}", ty.param_position(is_cb, state)?),
            Self::MutRef(ty) => format!("&mut {}", ty.param_position(is_cb, state)?),
            Self::Slice(ty) => format!("&[{}]", ty.param_position(is_cb, state)?),
            Self::Void => "todo_void".to_string(),
            Self::Array(ty, size) => format!("[{}; {}]", ty.param_position(is_cb, state)?, size),
            Self::Item(item) => match &**item {
                Item::AlreadyImported(name) => name.to_string(),
                Item::ConstantSet { ty, .. } => ty.param_position(is_cb, state)?,
                Item::FunctionType {
                    params,
                    return_type,
                    ..
                } => function_type(params, return_type.as_ref(), state)?,
                Item::ForeignHandle(name) => name.to_upper_camel_case(),
                Item::Interface => return Err(anyhow!("interface invalid in param position")),
                Item::Special(s) => s.name().to_string(),
                Item::Structure { name, .. } => {
                    if self.needs_lifetime(state)? {
                        format!("{}<'_>", AsUpperCamelCase(name))
                    } else {
                        name.to_upper_camel_case()
                    }
                }
            },
            Self::Callback { name, .. } => {
                // generic param with this one's name
                if is_cb {
                    return Err(anyhow!("cannot use callback in callback"));
                }
                name.to_upper_camel_case()
            }
            ty => return Err(anyhow!("Invalid type in param position: {:?}", ty)),
        })
    }

    pub fn win32_param_position(&self, state: &mut State<'_>) -> Result<String> {
        Ok(match self {
            Self::Primitive(n) => n.to_string(),
            Self::Array(ty, sz) => format!("[{}; {}]", ty.win32_param_position(state)?, sz),
            Self::Ref(ty) if matches!(&**ty, Self::String) => "PCSTR".to_string(),
            Self::Ref(ty) if matches!(&**ty, Self::OsString) => "PCWSTR".to_string(),
            Self::MutRef(ty) if matches!(&**ty, Self::String) => "PSTR".to_string(),
            Self::MutRef(ty) if matches!(&**ty, Self::OsString) => "PWSTR".to_string(),
            Self::Ref(ty) => format!("*const {}", ty.win32_param_position(state)?),
            Self::MutRef(ty) => format!("*mut {}", ty.win32_param_position(state)?),
            Self::Void => "c_void".to_string(),
            Self::String => "PSTR".to_string(),
            Self::OsString => "PWSTR".to_string(),
            Self::Slice(_) => return Err(anyhow!("invalid slice in win32 param position")),
            Self::Callback { .. } => {
                return Err(anyhow!("invalid callback in win32 param position"))
            }
            Self::Item(item) => match &**item {
                Item::AlreadyImported(n) | Item::ForeignHandle(n) => n.to_string(),
                Item::ConstantSet { ty, .. } => ty.win32_param_position(state)?,
                Item::FunctionType {
                    params,
                    return_type,
                    ..
                } => function_type(params, return_type.as_ref(), state)?,
                Item::Special(s) => s.win32_name().to_string(),
                Item::Structure { name, namespace, .. } => format!("{}::{}", format_namespace(namespace), name),
                Item::Interface => unreachable!(),
            },
            ty => return Err(anyhow!("Invalid type in Win32 param position: {:?}", ty)),
        })
    }

    /// The type for the return position.
    pub fn return_position(&self, state: &mut State<'_>) -> Result<String> {
        Ok(match self {
            Self::Primitive(name) => name.to_string(),
            Self::Array(ty, size) => format!("[{}; {}]", ty.return_position(state)?, size),
            Self::Ref(ty) => format!("&{}", ty.return_position(state)?),
            Self::MutRef(ty) => format!("&mut {}", ty.return_position(state)?),
            Self::Slice(ty) => return Err(anyhow!("slice invalid in return position")),
            Self::Void => "todo_void_ret".to_string(),
            Self::String => "CString".into(),
            Self::OsString => "OsString".into(),
            Self::Item(item) => match &**item {
                Item::AlreadyImported(name) => name.to_string(),
                Item::ForeignHandle(name) => name.to_upper_camel_case(),
                Item::ConstantSet { ty, .. } => ty.return_position(state)?,
                Item::FunctionType {
                    params,
                    return_type,
                    ..
                } => function_type(params, return_type.as_ref(), state)?,
                Item::Structure { name, .. } => {
                    if self.needs_lifetime(state)? {
                        format!("{}<'static>", AsUpperCamelCase(name))
                    } else {
                        name.to_upper_camel_case()
                    }
                }
                Item::Special(s) => s.name().to_string(),
                Item::Interface => return Err(anyhow!("Interface is invalid in return position")),
            },
            Self::Callback { .. } => return Err(anyhow!("callback invalid in return position")),
            ty => return Err(anyhow!("Invalid type in return position: {:?}", ty)),
        })
    }

    pub fn is_fallible(&self, fnname: &str, state: &mut State<'_>) -> bool {
        match self {
            Self::Primitive(name) if name == "i32" => true,
            Self::Item(item) => match &**item {
                Item::AlreadyImported("BOOL") => {
                    // if this is an actual boolean, then it's fallible
                    let fnname = fnname.to_snake_case();
                    for pattern in &state.actual_bool_globs {
                        if pattern.matches(&fnname) {
                            return false;
                        }
                    }

                    true
                }
                Item::ForeignHandle(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    /// Whether a type can be no-op casted to another.
    pub fn thin(&self, state: &mut State<'_>) -> Result<bool> {
        Ok(match self {
            Self::Primitive(_) => true,
            Self::Array(ty, _) => ty.thin(state)?,
            Self::Item(i) => match &**i {
                Item::AlreadyImported(_) => true,
                Item::ConstantSet { .. } => true,
                Item::ForeignHandle(_) => true,
                Item::FunctionType { .. } => false,
                Item::Special(_) => true,
                Item::Structure { fields, .. } => {
                    let first_field_name = &fields.first().unwrap().name;
                    let first_field_is_size =
                        first_field_name.ends_with("Size") || first_field_name.ends_with("size");

                    !first_field_is_size && 
                    all_res(fields
                        .iter()
                        .map(|f| {
                            let ty = f.ty(state)?;
                            ty.thin(state)
                        }))?
                }
                Item::Interface => return Err(anyhow!("interface invalid in thin")),
            },
            Self::Void => true,
            _ => false,
        })
    }

    /// From the name of the input expression, get an expression
    /// that can be used to get the Win32 value of the expression.
    pub fn input_expression(
        &self,
        input_name: &str,
        optional: bool,
        state: &mut State<'_>,
    ) -> Result<String> {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let tempvar = || format!("temp{}", COUNTER.fetch_add(1, Relaxed));
        let map_to_ptr = |name| {
            Ok(if optional {
                format!("{}.map_or(ptr::null(), |v| v.as_ptr() as _)", name)
            } else {
                format!("{}.as_ptr() as _", name)
            })
        };
        let os_string = |input_name, state: &mut State<'_>| {
            let name = tempvar();
            let mut source = input_name;

            if optional {
                swwriteln!(state, "let {} = {}.map(|val| {{", &name, input_name,)?;
                source = "val";
                state.bump_indent();
            }

            swwriteln!(
                state,
                "let mut {}: Vec<u16> = {}.encode_wide().collect();",
                &name,
                source,
            )?;
            swwriteln!(state, "{}.push(0);", &name)?;

            if optional {
                swwriteln!(state, "{}", &name)?;
                state.unbump_indent();
                swwriteln!(state, "}});")?;
            }

            map_to_ptr(name)
        };

        match self {
            Self::Primitive(_) => Ok(input_name.to_string()),
            Self::String => map_to_ptr(input_name.to_string()),
            Self::Ref(ty) if matches!(&**ty, Self::String) => map_to_ptr(input_name.to_string()),
            Self::Ref(ty) if matches!(&**ty, Self::OsString) => os_string(input_name, state),
            Self::Ref(ty) => {
                // if it's a thin type, we can do a direct cast
                if ty.thin(state)? {
                    swwriteln!(state, "// SAFETY: type is a thin type")?;
                    Ok(if optional {
                        format!(
                            "{}.map_or(ptr::null(), |val| unsafe {{ &*(val as *const _ as *const _) }})",
                            input_name,
                        )
                    } else {
                        format!("unsafe {{ &*({} as *const _ as *const _) }}", input_name,)
                    })
                } else {
                    let name = tempvar();
                    let ie = ty.input_expression(input_name, optional, state)?;
                    swwriteln!(state, "let {} = {};", &name, ie)?;

                    Ok(
                        format!("&{}", name)
                    )
                }
            }
            Self::MutRef(ty) => {
                let name = tempvar();
                let ie = ty.input_expression(input_name, optional, state)?;
                swwriteln!(state, "let mut {} = {};", &name, ie)?;

                Ok(if optional {
                    format!("{}.as_mut()", name)
                } else {
                    format!("&mut {}", name)
                })
            }
            Self::Slice(ty) => map_to_ptr(input_name.to_string()),
            Self::Array(ty, size) => {
                let size = *size;
                let mut expr = String::new();

                let mut source = input_name;
                if optional {
                    source = "val";
                    write!(expr, "{}.map(|val| {{ ", input_name).ok();
                }

                expr.push('[');

                for i in 0..size {
                    let input_expr = format!("{}[{}]", source, i);
                    write!(
                        expr,
                        "{}",
                        ty.input_expression(&input_expr, optional, state)?
                    )
                    .ok();

                    if i != size - 1 {
                        expr.push(',');
                    }
                }

                expr.push(']');
                if optional {
                    expr.push_str(" }})");
                }

                Ok(expr)
            }
            Self::OsString => os_string(input_name, state),
            Self::Void => Ok("todo_void".to_string()),
            Self::Item(item) => match &**item {
                Item::AlreadyImported(_) | Item::ConstantSet { .. } => Ok(input_name.to_string()),
                Item::ForeignHandle(ty) => Ok(if optional {
                    format!(
                        "{}.map_or(0, {}::into_raw)",
                        input_name,
                        AsUpperCamelCase(ty),
                    )
                } else {
                    format!("{}.into_raw()", input_name)
                }),
                Item::FunctionType { .. } => Ok(input_name.to_string()),
                Item::Special(s) => Ok(format!("{}.into_inner()", input_name)),
                Item::Structure { .. } => Ok(if optional {
                    format!(
                        "{}.map_or(unsafe {{ mem::zeroed() }}, |val| val.to_win32())",
                        input_name
                    )
                } else {
                    format!("{}.to_win32()", input_name)
                }),
                Item::Interface => Err(anyhow!("interface invalid in input_expression")),
            },
            Self::Callback {
                name,
                params,
                ret_ty,
                input_ptr_ty,
                fn_type,
            } => {
                // write the function that wraps around the main one and
                // calls it
                let fname = format!("{}_impl", AsSnakeCase(name),);
                let generic_name = name.to_upper_camel_case();
                swwrite!(state, r#"unsafe extern "system" fn {}<{}>("#, &fname, &generic_name)?;
                for (i, param) in params.iter().enumerate() {
                    let pp = param.win32_param_position(state)?;
                    swwrite!(state, "param{}: {}", i, pp)?;

                    if i != params.len() - 1 {
                        swwrite!(state, ", ",)?;
                    }
                }
                swwrite!(state, ") -> ")?;

                match ret_ty {
                    Some(ty) => {
                        // put in return position
                        let pp = ty.win32_param_position(state)?;
                        swwriteln!(state, "{} {{", pp,)?
                    }
                    None => {
                        swwriteln!(state, "() {{")?;
                    }
                }

                // write body
                state.indent(|state| {
                    // wrap it in a panic guard
                    // TODO: propogate the panic instead
                    swwriteln!(state, "abort_on_panic(move || {{")?;
                    state.bump_indent();
                    let iter_params = || {
                        params
                            .iter()
                            .enumerate()
                            .filter(|(_, p)| p.ptr_class() != Some(*input_ptr_ty))
                    };

                    // generate input parameters
                    for (i, param) in iter_params() {
                        let ret_expr = param.returnable_expr(&format!("param{}", i), &[], state)?;
                        swwriteln!(state, "let input{} = {};", i, ret_expr,)?;
                    }

                    // get the input pointer
                    let i_index = params
                        .iter()
                        .enumerate()
                        .find_map(|(index, p)| {
                            if p.ptr_class() == Some(*input_ptr_ty) {
                                Some(index)
                            } else {
                                None
                            }
                        })
                        .ok_or_else(|| anyhow!("input ptr type not found"))?;

                    swwriteln!(state, "let closure = unsafe {{")?;
                    state.indent(|state| {
                        swwriteln!(
                            state,
                            "{}(param{} as usize as {})",
                            match fn_type {
                                FnType::FnMut => "&mut *",
                            },
                            i_index,
                            match fn_type {
                                FnType::FnMut => format!("*mut {}", &generic_name),
                            }
                        )
                    })?;
                    swwriteln!(state, "}};")?;

                    // call the closure
                    swwrite!(state, "let return_value = (closure)(")?;

                    for (i, _) in iter_params() {
                        swwrite!(state, "input{}", i,)?;

                        if i != params.len() - 1 {
                            swwrite!(state, ", ")?;
                        }
                    }

                    swwriteln!(state, ");")?;

                    // prepare the return value to be returned
                    if let Some(ret_ty) = ret_ty {
                        let ret_val = ret_ty.input_expression("return_value", false, state)?;
                        swwriteln!(state, "let real_return_value = {};", ret_val,)?;
                    }

                    // return the final value
                    if ret_ty.is_some() {
                        swwriteln!(state, "real_return_value")?;
                    }

                    state.unbump_indent();
                    swwriteln!(state, "}})");

                    anyhow::Ok(())
                })?;
                swwriteln!(state, "}}")?;

                Ok(format!("Some({}::<{}>)", fname, &generic_name))
            }
            ty => return Err(anyhow!("Invalid type in input position: {:?}", ty)),
        }
    }

    /// Convert this item from the Win32 version to a returnable version.
    pub fn returnable_expr(
        &self,
        input: &str,
        params: &[Param],
        state: &mut State<'_>,
    ) -> Result<String> {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let tempvar = || format!("out{}", COUNTER.fetch_add(1, Relaxed));

        match self {
            Self::Primitive(_) => Ok(input.to_string()),
            Self::Ref(ty) => {
                // type needs to be thin
                if !ty.thin(state)? {
                    return Err(anyhow!("ref type is not thin"));
                }

                Ok(format!("unsafe {{ &*({} as *const _) }}", input))
            }
            Self::MutRef(ty) => {
                // type needs to be thin
                if !ty.thin(state)? {
                    return Err(anyhow!("mut ref type is not thin"));
                }

                Ok(format!("unsafe {{ &mut *({} as *mut _) }}", input))
            }
            Self::Array(ty, sz) => {
                let sz = *sz;

                // run for each individual item
                let mut expr = "[".to_string();

                for i in 0..sz {
                    let input_expr = format!("{}[{}]", input, i);
                    write!(expr, "{}", ty.returnable_expr(&input_expr, params, state)?).ok();

                    if i != sz - 1 {
                        expr.push(',');
                    }
                }

                expr.push(']');
                Ok(expr)
            }
            Self::Void => Ok("todo_void".to_string()),
            Self::String | Self::OsString => {
                // when returning a string, figure out which output parameter
                // represents the length
                let target_output = params
                    .iter()
                    .rev()
                    .filter(|p| matches!(p.variation, Variation::Output | Variation::InOut))
                    .find_map(|param| {
                        if likely_slice_count(&param.name) {
                            Some(param.name)
                        } else {
                            None
                        }
                    })
                    .unwrap_or("return_value");

                // read the string to a buffer
                swwriteln!(state, "{}.truncate({} as usize);", input, AsSnakeCase(target_output))?;

                // convert to cstring or osstring
                match self {
                    Self::String => {
                        Ok(format!(
                            "unsafe {{ CString::from_vec_unchecked({}) }}",
                            input
                        ))
                    }
                    Self::OsString => Ok(format!("OsString::from_wide(&{})", input)),
                    _ => unreachable!(),
                }
            }
            Self::Void => Ok("todo_void".to_string()),
            Self::Item(item) => match &**item {
                Item::AlreadyImported(_) | Item::ConstantSet { .. } => Ok(input.to_string()),
                Item::ForeignHandle(name) => Ok(format!(
                    "unsafe {{ {}::new({}) }}",
                    AsUpperCamelCase(name),
                    input
                )),
                Item::FunctionType { .. } => Ok(format!("Some({})", input)),
                Item::Special(s) => {
                    let class_name = s.name();

                    Ok(format!(
                        "unsafe {{ {}::from_inner({}) }}",
                        class_name, input
                    ))
                }
                Item::Structure { name, .. } => Ok(format!(
                    "unsafe {{ {}::from_win32({}) }}",
                    AsUpperCamelCase(name),
                    input
                )),
                Item::Interface => Err(anyhow!("interface invalid in returnable_expr")),
            },
            Self::Slice(_) => Err(anyhow!("invalid slice in return position")),
            Self::Callback { .. } => Err(anyhow!("invalid callback in return position")),
            ty => return Err(anyhow!("Invalid type in return position: {:?}", ty)),
        }
    }

    /// Convert this item to the Win32 equivalent.
    pub fn to_win32(&self, fname: &str, state: &mut State<'_>) -> Result<String> {
        Ok(match self {
            Type::Primitive(_) => {
                format!("*{}", fname)
            }
            ty if ty.is_str_ref() || ty.is_os_str_ref() => {
                // convert from string
                if ty.is_os_str_ref() {
                    // ensure last bit is zero
                    swwriteln!(state, "assert_eq!({}.last().copied(), Some(0));", fname,)?;
                }
                format!("{}.as_ptr() as _", fname)
            }
            Type::Ref(ty) => {
                if !ty.thin(state)? {
                    return Err(anyhow!("references to non-thin types are not supported"));
                }

                format!("unsafe {{ &*(&**{} as *const _ as *const _) }}", &fname)
            }
            Type::MutRef(ty) => {
                if !ty.thin(state)? {
                    return Err(anyhow!("references to non-thin types are not supported"));
                }

                format!("unsafe {{ &mut *({} as *mut _ as *mut _) }}", &fname)
            }
            Type::Array(ty, size) => {
                let mut expr = "[".to_string();
                let size = *size;
                let needs_underef = match &**ty {
                            Type::Primitive(_) => true,
                            Type::Item(item) => matches!(&**item, Item::AlreadyImported(_)),
                            _ => false,
                        };

                for i in 0..size {
                    let input_expr = format!(
                        "{}{}[{}]", 
                        if needs_underef {
                            "&"
                        } else {
                            ""
                        },
                        fname, 
                        i
                    );
                    write!(expr, "{}", ty.to_win32(&input_expr, state)?).ok();

                    if i != size - 1 {
                        expr.push(',');
                    }
                }

                expr.push(']');

                expr
            }
            Type::Void => "todo_void".to_string(),
            Type::Item(item) => match &**item {
                Item::AlreadyImported(_) | Item::ConstantSet { .. } => format!("*{}", fname),
                Item::ForeignHandle(class_name) => {
                    format!(
                        "{}.map_or(0, {}::into_raw)",
                        fname,
                        AsUpperCamelCase(class_name)
                    )
                }
                Item::FunctionType { .. } => format!("*{}", fname),
                Item::Special(_) => format!("{}.into_inner()", fname),
                Item::Structure { .. } => format!("{}.to_win32()", fname),
                Item::Interface => return Err(anyhow!("interface invalid in to_win32")),
            },
            Type::Slice(ty) => format!("({}).as_ptr()", ty.to_win32(fname, state)?),
            Type::Callback { .. } => return Err(anyhow!("callback invalid in field position")),
            _ => unreachable!(),
        })
    }

    /// Convert this item from its Win32 equivalent.
    pub fn from_win32(&self, fname: &str, state: &mut State<'_>) -> Result<String> {
        Ok(match self {
            Type::Primitive(_) => fname.to_string(),
            ty if ty.is_str_ref() => {
                // convert from string
                swwriteln!(state, "let {0} = unsafe {{ CStr::from_ptr({0} as *const _) }};", fname,)?;
                swwriteln!(state, "let {0} = {0}.to_bytes_with_nul().to_vec();", fname,)?;

                format!(
                    "Cow::Owned(unsafe {{ CString::from_vec_unchecked({}) }})",
                    fname
                )
            }
            ty if ty.is_os_str_ref() => {
                // convert from string
                swwriteln!(
                    state,
                    "let slen = unsafe {{ wide_strlen({} as *const u16) }};",
                    fname
                )?;
                swwriteln!(
                    state,
                    "let {0} = unsafe {{ slice::from_raw_parts({0}, slen + 1) }};",
                    fname,
                )?;

                format!("Cow::Owned({}.to_vec())", fname)
            }
            Type::Ref(ty) => {
                if !ty.thin(state)? {
                    return Err(anyhow!("references to non-thin types are not supported"));
                }

                format!("Cow::Borrowed(unsafe {{ &*({} as *const _ as *const _) }})", &fname)
            }
            Type::MutRef(ty) => {
                if !ty.thin(state)? {
                    return Err(anyhow!("references to non-thin types are not supported"));
                }

                format!("unsafe {{ &mut *({} as *mut _ as *mut _) }}", &fname)
            }
            Type::Array(ty, sz) => {
                let mut expr = "[".to_string();
                let sz = *sz;

                for i in 0..sz {
                    let input_expr = format!("{}[{}]", fname, i);
                    write!(expr, "{}", ty.from_win32(&input_expr, state)?).ok();

                    if i != sz - 1 {
                        expr.push(',');
                    }
                }

                expr.push(']');
                expr
            }
            Type::Void => "todo_void".to_string(),
            Type::Item(item) => match &**item {
                Item::AlreadyImported(_) | Item::ConstantSet { .. } => fname.to_string(),
                Item::ForeignHandle(name) => {
                    format!(
                        "unsafe {{ {}::new_optional({}) }}",
                        AsUpperCamelCase(name),
                        fname
                    )
                }
                Item::FunctionType { .. } => fname.to_string(),
                Item::Special(s) => {
                    let class_name = s.name();

                    format!("unsafe {{ {}::from_inner({}) }}", class_name, fname)
                }
                Item::Structure { name, namespace, .. } => {
                    format!("unsafe {{ {}::from_win32({}) }}", AsUpperCamelCase(name), fname)
                }
                Item::Interface => return Err(anyhow!("interface invalid in from_win32")),
            },
            _ => unreachable!(),
        })
    }

    pub fn ptr_class(&self) -> Option<PtrClass> {
        match self {
            Self::Primitive(prim) if prim == "usize" => Some(PtrClass::Usize),
            Self::Ref(ty) if matches!(&**ty, Self::Void) => Some(PtrClass::VoidPtr),
            Self::Item(it) => match &**it {
                Item::Special(Special::Lparam) => Some(PtrClass::Lparam),
                Item::Special(Special::Wparam) => Some(PtrClass::Wparam),
                _ => None,
            },
            _ => None,
        }
    }

    /// Does this type involve a varsized type?
    pub fn involves_varsized(&self) -> bool {
        matches!(self, Self::VariableSize(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtrClass {
    Wparam,
    Lparam,
    VoidPtr,
    Usize,
}

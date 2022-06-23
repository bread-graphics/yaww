// MIT/Apache2 License

#![forbid(unsafe_code)]

use anyhow::{anyhow, Context, Result};
use glob::Pattern;
use indenter::{indented, Indented};
use std::{
    borrow::Cow,
    cmp,
    collections::{BTreeMap, VecDeque},
    env,
    fmt::{self, Write},
    fs,
    io::{prelude::*, BufReader, BufWriter, Write as _},
    mem,
    rc::Rc,
};
use take_mut::take;
use windows_metadata::reader::{ConstantValue, MethodDef, Type, TypeDef};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // open files
    let mut args = env::args_os();
    let config_file = args.nth(1).unwrap_or_else(|| "../yaww.toml".into());
    let sw_output = args
        .next()
        .unwrap_or_else(|| "../safer-wingui/src/automatically-generated.rs".into());
    let y_output = args
        .next()
        .unwrap_or_else(|| "../yaww/src/automatically-generated.rs".into());

    let mut config_file = BufReader::new(fs::File::open(config_file)?);
    let mut sw_output = BufWriter::new(fs::File::create(sw_output)?);
    let mut y_output = BufWriter::new(fs::File::create(y_output)?);

    // read config to memory
    let mut config_buffer = vec![];
    config_file.read_to_end(&mut config_buffer)?;

    // create a new instance of the config struct
    let mut config: Config = toml::from_slice(&config_buffer)?;
    println!("{:?}", &config);

    // open a winapi reader
    let reader = windows_metadata::reader::TypeReader::get();

    // construct the state for producing types
    let modules = mem::take(&mut config.directives.modules);
    let mut sw_buffer = String::new();
    let mut y_buffer = String::new();
    let mut state = State::new(config, &mut sw_buffer, &mut y_buffer)?;

    // iterate over the modules we want
    for module in &modules {
        state.current_namespace = Some(&*module);

        // get the module
        let module = reader
            .get_namespace(module)
            .ok_or_else(|| anyhow!("Could not find module {}", module))?;

        // iterate over types
        for ty in module.types.iter().flat_map(|(_, tyv)| tyv) {
            state.handle_type(ty)?;
        }
    }

    state.handle_callbacks()?;
    state.handle_items()?;
    state.handle_methods()?;

    // write the output
    sw_output.write_all(sw_buffer.as_bytes())?;
    y_output.write_all(y_buffer.as_bytes())?;

    Ok(())
}

pub struct State<'s> {
    pub config: Config<'s>,
    pub ignore_globs: Vec<Pattern>,
    pub actual_bool_globs: Vec<Pattern>,

    sw_output: Indented<'s, String>,
    y_output: Indented<'s, String>,
    indent: usize,

    pub current_namespace: Option<&'s str>,
    pub items: BTreeMap<String, Rc<Item>>,
    pub callbacks_deferred: Vec<DeferredTypeDef<'s>>,
    pub methods_deferred: Vec<DeferredMethod<'s>>,

    pub current_item_name: Option<Cow<'static, str>>,
    pub counter: usize,
}

pub struct DeferredMethod<'s> {
    namespace: &'s str,
    method: &'static MethodDef,
}

pub struct DeferredTypeDef<'s> {
    namespace: &'s str,
    typedef: &'static TypeDef,
}

const INDENT: &str = "                                                                        ";

impl<'s> State<'s> {
    fn new(
        config: Config<'s>,
        sw_output: &'s mut String,
        y_output: &'s mut String,
    ) -> Result<Self> {
        Ok(Self {
            ignore_globs: config
                .directives
                .ignore
                .iter()
                .map(|p| Pattern::new(p))
                .collect::<Result<Vec<_>, _>>()?,
            actual_bool_globs: config
                .directives
                .actual_bools
                .iter()
                .map(|p| Pattern::new(p))
                .collect::<Result<Vec<_>, _>>()?,
            config,
            sw_output: indented(sw_output).with_str(""),
            y_output: indented(y_output).with_str(""),
            indent: 0,
            current_namespace: None,
            items: BTreeMap::new(),
            callbacks_deferred: Vec::new(),
            methods_deferred: Vec::new(),
            current_item_name: None,
            counter: 0,
        })
    }

    fn update_indent(&mut self) {
        let indentation = &INDENT[..self.indent * 4];
        take(&mut self.sw_output, |s| s.with_str(indentation));
        take(&mut self.y_output, |y| y.with_str(indentation));
    }

    fn bump_indent(&mut self) {
        self.indent += 1;
        self.update_indent();
    }

    fn unbump_indent(&mut self) {
        self.indent = cmp::max(0, self.indent - 1);
        self.update_indent();
    }

    pub fn indent<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.bump_indent();
        let r = f(self);
        self.unbump_indent();
        r
    }

    // get the item associated with the current type
    fn get_item(&mut self, ty: &Type) -> Result<types::Type> {
        match ty {
            Type::Void => Ok(types::Type::Void),
            Type::I8 => Ok(types::Type::Primitive("i8")),
            Type::I16 => Ok(types::Type::Primitive("i16")),
            Type::I32 => Ok(types::Type::Primitive("i32")),
            Type::I64 => Ok(types::Type::Primitive("i64")),
            Type::U8 => Ok(types::Type::Primitive("u8")),
            Type::U16 => Ok(types::Type::Primitive("u16")),
            Type::U32 => Ok(types::Type::Primitive("u32")),
            Type::U64 => Ok(types::Type::Primitive("u64")),
            Type::F32 => Ok(types::Type::Primitive("f32")),
            Type::F64 => Ok(types::Type::Primitive("f64")),
            Type::ISize => Ok(types::Type::Primitive("isize")),
            Type::USize => Ok(types::Type::Primitive("usize")),
            Type::String => Ok(types::Type::String),
            Type::Bool => Ok(types::Type::Primitive("bool")),
            Type::Char => Ok(types::Type::Primitive("u8")),
            Type::HRESULT => Ok(types::Type::Primitive("HRESULT")),
            Type::GUID => Ok(types::Type::Primitive("GUID")),
            Type::IUnknown => Ok(types::Type::Item(Rc::new(Item::Interface))),
            Type::TypeDef(td) => {
                // if this is a union, generate the anonymous types
                if td.is_union() {
                    let base_name = format!(
                        "{}_{}",
                        self.current_item_name.as_ref().unwrap(),
                        self.counter
                    );
                    self.counter += 1;

                    // generate the further types
                    let mut fields = vec![];
                    let mut anon_index = 0;
                    for (i, field) in td.fields().enumerate() {
                        let ty = field.get_type(Some(td));
                        let ty = match self.get_item(&ty) {
                            Ok(ty) => ty,
                            Err(_) => {
                                // create an ad-hoc struct type
                                let td = match ty {
                                    Type::TypeDef(td) => td,
                                    _ => {
                                        return Err(anyhow!("Expected a type def for union field"))
                                    }
                                };

                                let item = self.compose_struct(&td)?;
                                let new_name = format!("{}_{}", base_name, anon_index);
                                anon_index += 1;
                                let item = Rc::new(item);
                                self.items.insert(new_name, item.clone());
                                types::Type::Item(item)
                            }
                        };

                        fields.push(Field {
                            name: Cow::Owned(format!("field{}", i)),
                            ty: ResolveLater::resolved(ty),
                        });
                    }

                    let item = Rc::new(Item::Structure {
                        name: Cow::Owned(base_name.clone()),
                        is_union: true,
                        fields: fields,
                        namespace: self.current_namespace.unwrap().to_string(),
                        packing: None,
                    });
                    self.items.insert(base_name, item.clone());
                    return Ok(types::Type::Item(item));
                }

                self.items
                    .get(td.name())
                    .map(|item| types::Type::Item(item.clone()))
                    .ok_or_else(|| anyhow!("Could not find type {}", td.name()))
            }
            Type::ConstPtr((ty, depth)) => {
                let mut base = self.get_item(&*ty)?;
                for _ in 0..*depth {
                    base = types::Type::Ref(Box::new(base));
                }
                Ok(base)
            }
            Type::MutPtr((ty, depth)) => {
                let mut base = self.get_item(&*ty)?;
                for _ in 0..*depth {
                    base = types::Type::MutRef(Box::new(base));
                }
                Ok(base)
            }
            Type::PSTR => Ok(types::Type::MutRef(Box::new(types::Type::String))),
            Type::PCSTR => Ok(types::Type::Ref(Box::new(types::Type::String))),
            Type::PWSTR => Ok(types::Type::MutRef(Box::new(types::Type::OsString))),
            Type::PCWSTR => Ok(types::Type::Ref(Box::new(types::Type::OsString))),
            Type::Win32Array((ty, size)) => Ok(types::Type::Array(
                Box::new(self.get_item(ty)?),
                *size as usize,
            )),
            ty => Err(anyhow!("Unhandled type {:?}", DebugType::from(ty))),
        }
    }

    fn compose_struct(&mut self, td: &TypeDef) -> Result<Item> {
        self.current_item_name = Some(Cow::Borrowed(td.name()));

        let fields = td
            .fields()
            .map(|field| {
                let field_ty = field.get_type(Some(td));
                Field {
                    name: Cow::Borrowed(field.name()),
                    ty: match self.get_item(&field_ty) {
                        Ok(ty) => ResolveLater::resolved(ty),
                        Err(_) => ResolveLater::unresolved(field_ty),
                    },
                }
            })
            .collect();

        Ok(Item::Structure {
            name: Cow::Borrowed(td.name()),
            is_union: false,
            fields,
            namespace: self.current_namespace.unwrap().to_string(),
            packing: td.class_layout().map(|l| l.packing_size() as usize),
        })
    }

    fn item_for_typedef(&mut self, td: &'static TypeDef) -> Result<Option<Item>> {
        // skip interfaces
        let name = td.name();
        if name.starts_with('I') && name.to_uppercase() != name {
            tracing::info!("Skipping interface");
            return Ok(Some(Item::Interface));
        }

        Ok(if td.is_handle() {
            // if the first field is an isize, we can assume it's a foreign
            // object, otherwise it's a typedef and can be imported
            if td
                .fields()
                .next()
                .map(|f| f.get_type(Some(td)))
                .filter(|f| matches!(f, Type::ISize))
                .is_some()
            {
                Some(Item::ForeignHandle(td.name()))
            } else {
                let namespace = types::format_namespace(self.current_namespace.unwrap());
                swwriteln!(self, "pub use {}::{};", namespace, td.name())?;
                Some(Item::AlreadyImported(td.name()))
            }
        } else if td.is_primitive() {
            // create constant values
            let mut ty = None;
            let constants = td
                .fields()
                .filter_map(|f| f.constant().map(|c| (f.name(), c)))
                .map(|(name, c)| {
                    if let ty @ None = &mut ty {
                        *ty = Some(self.get_item(&c.value_type())?);
                    }
                    Ok(Constant {
                        name,
                        value: match c.value() {
                            ConstantValue::Bool(b) => Value::Boolean(b),
                            ConstantValue::String(s) => Value::String(s.to_string()),
                            ConstantValue::U8(u) => Value::Number(u as i128),
                            ConstantValue::U16(u) => Value::Number(u as i128),
                            ConstantValue::U32(u) => Value::Number(u as i128),
                            ConstantValue::U64(u) => Value::Number(u as i128),
                            ConstantValue::I8(i) => Value::Number(i as i128),
                            ConstantValue::I16(i) => Value::Number(i as i128),
                            ConstantValue::I32(i) => Value::Number(i as i128),
                            ConstantValue::I64(i) => Value::Number(i as i128),
                            ConstantValue::F32(f) => Value::Float(f as f64),
                            ConstantValue::F64(f) => Value::Float(f),
                            ConstantValue::TypeDef(_) => {
                                return Err(anyhow!("Constant value is a typedef"))
                            }
                        },
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Some(Item::ConstantSet {
                name: td.name(),
                constants,
                ty: ty
                    .ok_or_else(|| anyhow!("Could not find type for constant set {}", td.name()))?,
            })
        } else if td.is_callback() {
            // figure out how it's going to work
            self.callbacks_deferred.push(DeferredTypeDef {
                namespace: self.current_namespace.unwrap(),
                typedef: td,
            });
            None
        } else {
            Some(self.compose_struct(td)?)
        })
    }

    fn handle_type(&mut self, ty: &'static Type) -> Result<()> {
        let (name, item) = match ty {
            Type::TypeDef(td) => {
                tracing::info!("Processing Typedef: {}", td.name());

                for pattern in self.ignore_globs.iter() {
                    if pattern.matches(td.name()) {
                        tracing::info!("Unmatched by ignore: {}", td.name());
                        return Ok(());
                    }
                }

                match self
                    .item_for_typedef(td)
                    .with_context(|| format!("While processing type {:?}", DebugType::from(ty)))?
                {
                    Some(ni) => (td.name(), ni),
                    None => return Ok(()),
                }
            }
            Type::MethodDef(md) => {
                self.methods_deferred.push(DeferredMethod {
                    namespace: self.current_namespace.unwrap(),
                    method: md,
                });
                return Ok(());
            }
            _ => return Ok(()),
        };

        self.items.insert(name.to_string(), Rc::new(item));

        Ok(())
    }

    fn handle_items(&mut self) -> Result<()> {
        let items = self.items.values().cloned().collect::<Vec<_>>();
        for item in items {
            self.current_item_name = Some(item.name());
            item.write(self)
                .with_context(|| format!("Handling item {:#?}", item))?;
        }

        Ok(())
    }

    fn handle_callbacks(&mut self) -> Result<()> {
        'l: for td in mem::take(&mut self.callbacks_deferred) {
            let DeferredTypeDef {
                namespace,
                typedef: td,
            } = td;

            // if it's COM, ignore it
            for cfg_feature in td.cfg().features(namespace) {
                if cfg_feature.ends_with("Com") || cfg_feature.ends_with("Ole") {
                    continue 'l;
                }
            }

            // figure out params + return type
            let signature = td.invoke_method().signature(&[]);
            let return_type = signature
                .return_type
                .as_ref()
                .map(|ty| self.get_item(ty))
                .transpose()
                .with_context(|| format!("finding return type for {}", td.name()))?;
            let params = signature
                .params
                .iter()
                .map(|param| {
                    self.get_item(&param.ty).map(|ty| {
                        Param::new(
                            param.def.name(),
                            ty,
                            param.def.flags().into(),
                            param.def.flags().optional(),
                        )
                    })
                })
                .collect::<Result<_>>()
                .with_context(|| format!("finding params for {}", td.name()))?;

            self.items.insert(
                td.name().to_string(),
                Rc::new(Item::FunctionType {
                    name: td.name(),
                    params,
                    return_type,
                }),
            );
        }

        Ok(())
    }

    fn handle_methods(&mut self) -> Result<()> {
        'l: for md in mem::take(&mut self.methods_deferred) {
            let DeferredMethod {
                namespace,
                method: md,
            } = md;

            // do we ignore this
            for pattern in &self.ignore_globs {
                if pattern.matches(md.name()) {
                    continue 'l;
                }
            }

            // if it's COM we also ignore it
            for cfg_feature in md.cfg().features(namespace) {
                if cfg_feature.contains("Com") {
                    continue 'l;
                }
            }

            // figure out the types we need
            let signature = md.signature(&[]);
            let return_type = signature
                .return_type
                .as_ref()
                .map(|ty| self.get_item(ty))
                .transpose()
                .with_context(|| format!("finding return type for {}", md.name()))?;
            let params = signature
                .params
                .iter()
                .map(|param| {
                    self.get_item(&param.ty).map(|ty| {
                        Param::new(
                            param.def.name(),
                            ty,
                            param.def.flags().into(),
                            param.def.flags().optional(),
                        )
                    })
                })
                .collect::<Result<Vec<_>>>()
                .with_context(|| format!("finding params for {}", md.name()))?;

            // skip if this involves interfaces in any way
            if return_type
                .iter()
                .chain(params.iter().map(|p| &p.ty))
                .any(|ty| ty.is_interface())
            {
                continue 'l;
            }

            let function = Function {
                name: md.name(),
                params,
                return_type,
                namespace: namespace.to_string(),
            };
            function
                .write(self)
                .with_context(|| format!("Writing function {}", md.name()))?;
        }

        Ok(())
    }

    pub fn write_sw(&mut self, args: fmt::Arguments<'_>) -> Result<()> {
        self.sw_output.write_fmt(args)?;
        Ok(())
    }

    pub fn write_y(&mut self, args: fmt::Arguments<'_>) -> Result<()> {
        self.y_output.write_fmt(args)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! swwrite {
    ($state: expr, $($tt: tt)*) => {
        ($state).write_sw(format_args!($($tt)*))
    }
}

#[macro_export]
macro_rules! swwriteln {
    ($state: expr) => {
        swwrite!($state, "\n")
    };
    ($state: expr, $($tt: tt)*) => {
        swwrite!($state, "{}\n", format_args!($($tt)*))
    }
}

#[macro_export]
macro_rules! ywrite {
    ($state: expr, $($tt: tt)*) => {
        ($state).write_y(format_args!($($tt)*))
    }
}

#[macro_export]
macro_rules! ywriteln {
    ($state: expr) => {
        ywrite!($state, "\n")
    };
    ($state: expr, $($tt: tt)*) => {
        ywrite!($state, "{}\n", format_args!($($tt)*))
    };
}

pub struct DebugType<'a>(Cow<'a, Type>);

impl<'a> From<Type> for DebugType<'a> {
    fn from(ty: Type) -> Self {
        DebugType(Cow::Owned(ty))
    }
}

impl<'a> From<&'a Type> for DebugType<'a> {
    fn from(ty: &'a Type) -> Self {
        DebugType(Cow::Borrowed(ty))
    }
}

impl<'a> fmt::Debug for DebugType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.0 {
            Type::Void => f.write_str("void"),
            Type::Bool => f.write_str("primitive: bool"),
            Type::Char => f.write_str("primitive: char"),
            Type::I8 => f.write_str("primitive: i8"),
            Type::I16 => f.write_str("primitive: i16"),
            Type::I32 => f.write_str("primitive: i32"),
            Type::I64 => f.write_str("primitive: i64"),
            Type::U8 => f.write_str("primitive: u8"),
            Type::U16 => f.write_str("primitive: u16"),
            Type::U32 => f.write_str("primitive: u32"),
            Type::U64 => f.write_str("primitive: u64"),
            Type::F32 => f.write_str("primitive: f32"),
            Type::F64 => f.write_str("primitive: f64"),
            Type::ISize => f.write_str("primitive: isize"),
            Type::USize => f.write_str("primitive: usize"),
            Type::String => f.write_str("primitive: string"),
            Type::GUID => f.write_str("special: guid"),
            Type::IUnknown => f.write_str("special: iunknown"),
            Type::IInspectable => f.write_str("special: iinspectable"),
            Type::HRESULT => f.write_str("pointer: hresult"),
            Type::PSTR => f.write_str("pointer: pstr"),
            Type::PWSTR => f.write_str("pointer: pwstr"),
            Type::PCSTR => f.write_str("pointer: pcstr"),
            Type::PCWSTR => f.write_str("pointer: pcwstr"),
            Type::TypeName => f.write_str("special: typename"),
            Type::GenericParam(n) => write!(f, "generic: {}", n),
            Type::MethodDef(md) => write!(f, "method def: {}", md.name()),
            Type::Field(fd) => write!(f, "field: {}", fd.name()),
            Type::TypeDef(td) => write!(f, "type def: {}", td.name()),
            Type::MutPtr((ref ty, depth)) => {
                write!(f, "mut ptr: {:?} ({})", DebugType::from(&**ty), depth)
            }
            Type::ConstPtr((ref ty, depth)) => {
                write!(f, "const ptr: {:?} ({})", DebugType::from(&**ty), depth)
            }
            Type::Win32Array((ref ty, size)) => {
                write!(f, "win32 array: {:?} ({})", DebugType::from(&**ty), size)
            }
            Type::WinrtArray(ref ty) => write!(f, "winrt array: {:?}", DebugType::from(&**ty)),
            Type::WinrtArrayRef(ref ty) => {
                write!(f, "winrt array ref: {:?}", DebugType::from(&**ty))
            }
            Type::WinrtConstRef(ref ty) => {
                write!(f, "winrt const ref: {:?}", DebugType::from(&**ty))
            }
        }
    }
}

mod config;
pub use config::*;
mod types;
pub use types::*;

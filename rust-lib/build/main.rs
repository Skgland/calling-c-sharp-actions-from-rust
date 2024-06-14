use std::fmt::Display;
use std::io::Write;
use std::{collections::HashSet, error::Error, time::Duration};
use syn::FnArg;

fn main() -> Result<(), Box<dyn Error>> {
    csbindgen::Builder::default()
        .input_extern_file("src/c_sharp.rs")
        .csharp_dll_name("rust_lib")
        .always_included_types(["SuccessAction"])
        .generate_csharp_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/dotnet/NativeMethods.g.cs"
        ))?;

    generate_boiler_plate()?;
    std::thread::sleep(Duration::from_secs(1));

    Ok(())
}

fn warn<S: Display>(msg: S) {
    println!("cargo:warning={msg}");
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Ptr,
}

impl Type {
    fn cs_name(&self) -> &str {
        match self {
            Type::I8 => "sbyte",
            Type::U8 => "byte",
            Type::I16 => "short",
            Type::U16 => "ushort",
            Type::I32 => "int",
            Type::U32 => "uint",
            Type::I64 => "long",
            Type::U64 => "ulong",
            Type::Ptr => "IntPtr",
        }
    }
}

impl TryFrom<syn::Type> for Type {
    type Error = ();

    fn try_from(value: syn::Type) -> Result<Self, Self::Error> {
        match value {
            syn::Type::Array(_) => Err(()),
            syn::Type::BareFn(_) => Ok(Self::Ptr),
            syn::Type::Group(tg) => Self::try_from(*tg.elem),
            syn::Type::Paren(tp) => Self::try_from(*tp.elem),
            syn::Type::Path(tp) => {
                let last_segment = tp.path.segments.last().ok_or(())?;
                if last_segment.ident == "i8" {
                    Ok(Self::I8)
                } else if last_segment.ident == "u8" {
                    Ok(Self::U8)
                } else if last_segment.ident == "u16" {
                    Ok(Self::U16)
                } else if last_segment.ident == "i16" {
                    Ok(Self::I16)
                } else if last_segment.ident == "u32" {
                    Ok(Self::U32)
                } else if last_segment.ident == "i32" {
                    Ok(Self::I32)
                } else if last_segment.ident == "u64" {
                    Ok(Self::U64)
                } else if last_segment.ident == "i64" {
                    Ok(Self::I64)
                } else {
                    Err(())
                }
            }
            syn::Type::Ptr(_) => Ok(Self::Ptr),
            syn::Type::Reference(_) => Ok(Self::Ptr),
            _ => Err(()),
        }
    }
}

fn generate_boiler_plate() -> Result<(), Box<dyn Error>> {
    let file = syn::parse_file(include_str!("../src/c_sharp.rs"))?;

    let mut to_generate_cast = vec![];

    for item in file.items {
        if let syn::Item::Macro(item) = item {
            if item.ident.is_none()
                && item
                    .mac
                    .path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == "delegate")
            {
                let sig = syn::parse2::<syn::Signature>(item.mac.tokens)?;

                let name = sig.ident.to_string();

                let mut argument_types = vec![];
                for arg in sig.inputs {
                    match arg {
                        FnArg::Receiver(_) => continue,
                        FnArg::Typed(arg) => {
                            if let Ok(arg) = Type::try_from(syn::Type::clone(&arg.ty)) {
                                argument_types.push(arg);
                            } else {
                                warn(format!(
                                    "Skipping {name} due to unsupported argument type {:?}",
                                    arg.ty
                                ));
                                continue;
                            }
                        }
                    }
                }

                let ret = match sig.output {
                    syn::ReturnType::Default => None,
                    syn::ReturnType::Type(_, ret) => {
                        if let Ok(ty) = Type::try_from(syn::Type::clone(&ret)) {
                            Some(ty)
                        } else {
                            warn(format!(
                                "Skipping {name} due to unsupported return type {ret:?}",
                            ));
                            continue;
                        }
                    }
                };

                to_generate_cast.push((name, argument_types, ret));
            }
        }
    }

    const MAIN_TEMPLATE: &str = include_str!("NativeMethods.main.cs.template");

    let content = MAIN_TEMPLATE.replace(
        "@@casts@@",
        &to_generate_cast
            .iter()
            .map(|(name, args, ret)| generate_cast_boiler_plate(name, args, ret))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    let to_generate_create = to_generate_cast
        .into_iter()
        .map(|(_, args, ret)| (args, ret))
        .collect::<HashSet<_>>();

    let content = content.replace(
        "@@delegates@@",
        &to_generate_create
            .iter()
            .map(|(args, ret)| generate_create_boiler_plate(args, ret))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    let mut out_file = std::fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/dotnet/NativeMethods.g2.cs"
        ))?;

    writeln!(out_file, "{content}")?;

    Ok(())
}

fn generate_create_boiler_plate(args: &[Type], ret: &Option<Type>) -> String {
    const CREATE_TEMPLATE: &str = include_str!("NativeMethods.create.cs.template");

    let arg_types = args.iter().map(|arg| arg.cs_name()).collect::<Vec<_>>();
    let arg_vars = (0..)
        .map(|elem| format!("v{elem}"))
        .take(args.len())
        .collect::<Vec<_>>();

    let callback_pattern_types = arg_vars
        .iter()
        .zip(arg_types.iter())
        .map(|(var, arg)| format!(", {arg} {var}"))
        .collect::<String>();

    let callback_args = arg_vars.join(", ");

    let callback_types = arg_types
        .iter()
        .map(|arg| format!(", {arg}"))
        .collect::<String>();

    let callback_return = ret.as_ref().map_or("void", |ret| ret.cs_name());

    let kind = generate_cs_delegate_kind(args, ret);

    CREATE_TEMPLATE
        .replace("@@kind@@", &kind)
        .replace("@@callback-pattern-types@@", &callback_pattern_types)
        .replace("@@callback-args@@", &callback_args)
        .replace("@@callback-types@@", &callback_types)
        .replace("@@callback-return@@", callback_return)
        .replace("@@return@@", if ret.is_none() { "" } else { "return" })
}

fn generate_cs_delegate_kind(args: &[Type], ret: &Option<Type>) -> String {
    if let Some(ret) = &ret {
        format!(
            "Func<{}{}>",
            args.iter()
                .map(|arg| format!("{}, ", arg.cs_name()))
                .collect::<String>(),
            ret.cs_name()
        )
    } else {
        if args.is_empty() {
            String::from("Action")
        } else {
            format!(
                "Action<{}>",
                args.iter()
                    .map(|arg| arg.cs_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

fn generate_cast_boiler_plate(name: &str, args: &[Type], ret: &Option<Type>) -> String {
    const CAST_TEMPLATE: &str = include_str!("NativeMethods.cast.cs.template");

    let kind = generate_cs_delegate_kind(args, ret);

    CAST_TEMPLATE
        .replace("@@type@@", &name)
        .replace("@@kind@@", &kind)
}

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Attribute, Expr, ExprLit, Field, GenericArgument, Ident, Lit, Meta, Path, PathArguments,
    Result, Type,
};

#[allow(unused)]
pub struct ContainerInfo {
    pub into: Option<Path>,
    pub try_from: Option<Path>,
    pub debug: bool,
}

pub fn container_info(attrs: &[Attribute]) -> Result<ContainerInfo> {
    let mut into = None;
    let mut try_from = None;
    let mut debug = false;

    for attr in attrs {
        if !attr.path().is_ident("dynamic") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("debug") {
                debug = true;
                return Ok(());
            }
            if meta.path.is_ident("into") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(s) = s {
                    into = Some(s.parse()?);
                    return Ok(());
                }
            }
            if meta.path.is_ident("try_from") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(s) = s {
                    try_from = Some(s.parse()?);
                    return Ok(());
                }
            }
            Err(meta.error("unsupported attribute"))
        })?;
    }

    Ok(ContainerInfo {
        into,
        try_from,
        debug,
    })
}

pub enum DefValue {
    None,
    Default,
    Path(Path),
}

#[derive(Debug)]
pub enum ContainerType {
    None,
    Option,
    Vec,
    Map,
}

#[allow(unused)]
pub struct FieldInfo<'a> {
    pub field: &'a Field,
    pub type_name: String,
    pub name: String,
    pub skip: bool,
    pub flatten: bool,
    pub allow_default: DefValue,
    pub into: Option<Path>,
    pub try_from: Option<Path>,
    pub deprecated: Option<String>,
    pub validate: Option<Path>,
    pub doc: String,
    pub container_type: ContainerType,
}

impl FieldInfo<'_> {
    pub fn to_option(&self) -> TokenStream {
        let name = &self.name;
        let doc = &self.doc;
        let type_name = &self.type_name;
        let container_type = Ident::new(&format!("{:?}", self.container_type), Span::call_site());
        let get_default = if let Some(def) = self.compute_default() {
            quote!(Some(|| #def.to_dynamic()))
        } else {
            quote!(None)
        };
        quote!(
            crate::meta::ConfigOption {
                name: #name,
                doc: #doc,
                tags: &[],
                container: crate::meta::ConfigContainer::#container_type,
                type_name: #type_name,
                default_value: #get_default,
                possible_values: &[],
                fields: &[],
            }
        )
    }

    fn compute_default(&self) -> Option<TokenStream> {
        let ty = &self.field.ty;
        match &self.allow_default {
            DefValue::Default => Some(quote!(
                <#ty>::default()
            )),
            DefValue::Path(default) => Some(quote!(
                #default()
            )),
            DefValue::None => None,
        }
    }
}

const fn extract_lit_str_from_expr(expr: &Expr) -> Option<&syn::LitStr> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = expr
    {
        Some(s)
    } else {
        None
    }
}

pub fn field_info(field: &Field) -> Result<FieldInfo<'_>> {
    let mut name = field.ident.as_ref().unwrap().to_string();
    let mut skip = false;
    let mut flatten = false;
    let mut allow_default = DefValue::None;
    let mut try_from = None;
    let mut validate = None;
    let mut into = None;
    let mut deprecated = None;
    let mut doc = String::new();
    let mut container_type = ContainerType::None;

    let type_name = match &field.ty {
        Type::Path(p) => {
            let last_seg = p.path.segments.last().unwrap();
            match &last_seg.arguments {
                PathArguments::None => last_seg.ident.to_string(),
                PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                    let arg = args.args.first().unwrap();
                    match arg {
                        GenericArgument::Type(Type::Path(t)) => {
                            container_type = match last_seg.ident.to_string().as_str() {
                                "Option" => ContainerType::Option,
                                "Vec" => ContainerType::Vec,
                                _ => panic!("unhandled type for {name}: {:#?}", field.ty),
                            };
                            t.path.segments.last().unwrap().ident.to_string()
                        }
                        _ => panic!("unhandled type for {name}: {:#?}", field.ty),
                    }
                }
                PathArguments::AngleBracketed(args) if args.args.len() == 2 => {
                    let arg = args.args.last().unwrap();
                    match arg {
                        GenericArgument::Type(Type::Path(t)) => {
                            container_type = match last_seg.ident.to_string().as_str() {
                                "HashMap" => ContainerType::Map,
                                _ => panic!("unhandled type for {name}: {:#?}", field.ty),
                            };
                            t.path.segments.last().unwrap().ident.to_string()
                        }
                        _ => panic!("unhandled type for {name}: {:#?}", field.ty),
                    }
                }
                _ => panic!("unhandled type for {name}: {:#?}", field.ty),
            }
        }
        _ => panic!("unhandled type for {name}: {:#?}", field.ty),
    };

    for attr in &field.attrs {
        // Handle #[doc = "..."] attributes
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(nv) = &attr.meta {
                if let Some(s) = extract_lit_str_from_expr(&nv.value) {
                    if !doc.is_empty() {
                        doc.push('\n');
                    }
                    doc.push_str(&s.value());
                }
            }
            continue;
        }

        if !attr.path().is_ident("dynamic") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(s) = s {
                    name = s.value();
                    return Ok(());
                }
            }
            if meta.path.is_ident("default") {
                if meta.input.peek(syn::Token![=]) {
                    let value = meta.value()?;
                    let s: Lit = value.parse()?;
                    if let Lit::Str(s) = s {
                        allow_default = DefValue::Path(s.parse()?);
                        return Ok(());
                    }
                } else {
                    allow_default = DefValue::Default;
                    return Ok(());
                }
            }
            if meta.path.is_ident("deprecated") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(s) = s {
                    deprecated.replace(s.value());
                    return Ok(());
                }
            }
            if meta.path.is_ident("into") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(s) = s {
                    into = Some(s.parse()?);
                    return Ok(());
                }
            }
            if meta.path.is_ident("try_from") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(s) = s {
                    try_from = Some(s.parse()?);
                    return Ok(());
                }
            }
            if meta.path.is_ident("validate") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(s) = s {
                    validate = Some(s.parse()?);
                    return Ok(());
                }
            }
            if meta.path.is_ident("skip") {
                skip = true;
                return Ok(());
            }
            if meta.path.is_ident("flatten") {
                flatten = true;
                return Ok(());
            }
            Err(meta.error("unsupported attribute"))
        })?;
    }

    Ok(FieldInfo {
        field,
        type_name,
        name,
        skip,
        flatten,
        allow_default,
        into,
        try_from,
        deprecated,
        validate,
        doc,
        container_type,
    })
}

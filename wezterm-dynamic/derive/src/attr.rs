use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Field, Lit, Path, Result};

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

pub struct FieldInfo<'a> {
    pub field: &'a Field,
    pub name: String,
    pub skip: bool,
    pub flatten: bool,
    pub allow_default: DefValue,
    pub into: Option<Path>,
    pub try_from: Option<Path>,
    pub deprecated: Option<String>,
    pub validate: Option<Path>,
}

impl FieldInfo<'_> {
    pub fn to_dynamic(&self) -> TokenStream {
        let name = &self.name;
        let ident = &self.field.ident;
        if self.skip {
            quote!()
        } else if self.flatten {
            quote!(
                self.#ident.place_dynamic(place);
            )
        } else if let Some(into) = &self.into {
            quote!(
                let target : #into = (&self.#ident).into();
                place.insert(#name.to_dynamic(), target.to_dynamic());
            )
        } else {
            quote!(
                place.insert(#name.to_dynamic(), self.#ident.to_dynamic());
            )
        }
    }

    pub fn build_from_dynamic(&self, struct_name: &str) -> TokenStream {
        let name = &self.name;
        let ident = &self.field.ident;
        let ty = &self.field.ty;

        let check_deprecated = if let Some(reason) = &self.deprecated {
            quote!(
                wezterm_dynamic::Error::raise_deprecated_fields(options, #struct_name, #name, #reason)?;
            )
        } else {
            quote!()
        };
        let validate_value = if let Some(validator) = &self.validate {
            quote!(
                #validator(&value).map_err(|msg| {
                    wezterm_dynamic::Error::ErrorInField{
                        type_name: #struct_name,
                        field_name: #name,
                        error: msg,
                    }
                })?;
            )
        } else {
            quote!()
        };

        if self.skip {
            quote!()
        } else if self.flatten {
            quote!(
                #ident:
                    <#ty>::from_dynamic(value, options)
                            .map_err(|source| source.field_context(
                                #struct_name,
                                #name,
                                obj))?,
            )
        } else if let Some(try_from) = &self.try_from {
            match &self.allow_default {
                DefValue::Default => {
                    quote!(
                        #ident: match obj.get_by_str(#name) {
                            Some(v) => {
                                use core::convert::TryFrom;
                                #check_deprecated
                                let target = <#try_from>::from_dynamic(v, options)
                                    .map_err(|source| source.field_context(
                                        #struct_name,
                                        #name,
                                        obj,
                                    ))?;
                                let value = <#ty>::try_from(target)
                                    .map_err(|source| wezterm_dynamic::Error::ErrorInField{
                                        type_name:#struct_name,
                                        field_name:#name,
                                        error: format!("{:#}", source)
                                    })?;
                                #validate_value
                                value
                            }
                            None => {
                                <#ty>::default()
                            }
                        },
                    )
                }
                DefValue::Path(default) => {
                    quote!(
                        #ident: match obj.get_by_str(&#name) {
                            Some(v) => {
                                use core::convert::TryFrom;
                                #check_deprecated
                                let target = <#try_from>::from_dynamic(v, options)
                                    .map_err(|source| source.field_context(
                                        #struct_name,
                                        #name,
                                        obj,
                                    ))?;
                                let value = <#ty>::try_from(target)
                                    .map_err(|source| wezterm_dynamic::Error::ErrorInField{
                                        type_name:#struct_name,
                                        field_name:#name,
                                        error: format!("{:#}", source),
                                    })?;
                                #validate_value
                                value
                            }
                            None => {
                                #default()
                            }
                        },
                    )
                }
                DefValue::None => {
                    quote!(
                        #ident: {
                            use core::convert::TryFrom;
                            let target = <#try_from>::from_dynamic(obj.get_by_str(#name).map(|v| {
                                #check_deprecated
                                v
                            }).unwrap_or(&Value::Null), options)
                                    .map_err(|source| source.field_context(
                                        #struct_name,
                                        #name,
                                        obj,
                                    ))?;
                            let value = <#ty>::try_from(target)
                                    .map_err(|source| wezterm_dynamic::Error::ErrorInField{
                                        type_name:#struct_name,
                                        field_name:#name,
                                        error: format!("{:#}", source),
                                    })?;
                            #validate_value
                            value
                        },
                    )
                }
            }
        } else {
            match &self.allow_default {
                DefValue::Default => {
                    quote!(
                        #ident: match obj.get_by_str(#name) {
                            Some(v) => {
                                #check_deprecated
                                let value = <#ty>::from_dynamic(v, options)
                                    .map_err(|source| source.field_context(
                                        #struct_name,
                                        #name,
                                        obj,
                                    ))?;
                                #validate_value
                                value
                            }
                            None => {
                                <#ty>::default()
                            }
                        },
                    )
                }
                DefValue::Path(default) => {
                    quote!(
                        #ident: match obj.get_by_str(#name) {
                            Some(v) => {
                                #check_deprecated
                                let value = <#ty>::from_dynamic(v, options)
                                    .map_err(|source| source.field_context(
                                        #struct_name,
                                        #name,
                                        obj,
                                    ))?;
                                #validate_value
                                value
                            }
                            None => {
                                #default()
                            }
                        },
                    )
                }
                DefValue::None => {
                    quote!(
                        #ident: {
                            let value = <#ty>::from_dynamic(
                                    obj.get_by_str(#name).map(|v| {
                                        #check_deprecated
                                        v
                                    }).
                                    unwrap_or(&Value::Null),
                                    options
                                )
                                .map_err(|source| source.field_context(#struct_name, #name, obj))?;
                            #validate_value
                            value
                        },
                    )
                }
            }
        }
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

    for attr in &field.attrs {
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
        name,
        skip,
        flatten,
        allow_default,
        into,
        try_from,
        deprecated,
        validate,
    })
}

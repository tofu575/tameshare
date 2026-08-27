use syn::{Data, DeriveInput, Fields, LitInt, Type, spanned::Spanned};

/// TypedString deriveへ指定された文字数・trim設定を保持する。
struct TypedStringOptions {
    min: usize,
    max: usize,
    trim: bool,
}

/// TypedString deriveの入力形式を検証して実装Tokenを構築する。
pub fn typed_string_macro_impl(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    let visibility = &input.vis;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "TypedString does not support generic structs",
        ));
    }

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            input.span(),
            "TypedString can only be derived for tuple structs",
        ));
    };
    let Fields::Unnamed(fields) = &data.fields else {
        return Err(syn::Error::new(
            data.fields.span(),
            "TypedString requires a tuple struct with one String field",
        ));
    };
    if fields.unnamed.len() != 1 || !is_string(&fields.unnamed[0].ty) {
        return Err(syn::Error::new(
            fields.span(),
            "TypedString requires exactly one String field",
        ));
    }

    let options = parse_options(&input)?;
    let min = options.min;
    let max = options.max;
    let error_name = quote::format_ident!("{struct_name}Error");
    let normalize = if options.trim {
        quote::quote!(let value = value.trim().to_owned();)
    } else {
        quote::quote!(let value = value;)
    };

    Ok(quote::quote! {
        impl #struct_name {
            pub const MIN_CHARS: usize = #min;
            pub const MAX_CHARS: usize = #max;

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl ::std::convert::TryFrom<::std::string::String> for #struct_name {
            type Error = #error_name;

            fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, Self::Error> {
                #normalize
                let character_count = value.chars().count();
                if !(Self::MIN_CHARS..=Self::MAX_CHARS).contains(&character_count) {
                    return Err(#error_name);
                }
                Ok(Self(value))
            }
        }

        impl ::std::fmt::Display for #struct_name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #visibility struct #error_name;

        impl ::std::fmt::Display for #error_name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    formatter,
                    "{} must contain between {} and {} characters",
                    stringify!(#struct_name),
                    #struct_name::MIN_CHARS,
                    #struct_name::MAX_CHARS,
                )
            }
        }

        impl ::std::error::Error for #error_name {}
    })
}

/// derive属性から必須の文字数制約と任意のtrim指定を読み取る。
fn parse_options(input: &DeriveInput) -> syn::Result<TypedStringOptions> {
    let attributes: Vec<_> = input
        .attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("typed_string"))
        .collect();
    if attributes.len() != 1 {
        return Err(syn::Error::new(
            input.span(),
            "TypedString requires exactly one #[typed_string(min = ..., max = ...)] attribute",
        ));
    }

    let mut min = None;
    let mut max = None;
    let mut trim = false;
    attributes[0].parse_nested_meta(|meta| {
        if meta.path.is_ident("min") {
            if min.is_some() {
                return Err(meta.error("duplicate min option"));
            }
            min = Some(meta.value()?.parse::<LitInt>()?.base10_parse::<usize>()?);
            return Ok(());
        }
        if meta.path.is_ident("max") {
            if max.is_some() {
                return Err(meta.error("duplicate max option"));
            }
            max = Some(meta.value()?.parse::<LitInt>()?.base10_parse::<usize>()?);
            return Ok(());
        }
        if meta.path.is_ident("trim") {
            if trim {
                return Err(meta.error("duplicate trim option"));
            }
            trim = true;
            return Ok(());
        }
        Err(meta.error("unsupported TypedString option"))
    })?;

    let min = min.ok_or_else(|| syn::Error::new(input.span(), "TypedString requires min"))?;
    let max = max.ok_or_else(|| syn::Error::new(input.span(), "TypedString requires max"))?;
    if min > max {
        return Err(syn::Error::new(
            input.span(),
            "TypedString min must be less than or equal to max",
        ));
    }

    Ok(TypedStringOptions { min, max, trim })
}

/// tuple structの内包型がStringか判定する。
fn is_string(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "String")
}

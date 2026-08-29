use syn::{Data, DeriveInput, Fields, Type, spanned::Spanned};

/// TypedUuid deriveの入力形式を検証して実装Tokenを構築する。
pub fn typed_uuid_macro_impl(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "TypedUuid does not support generic structs",
        ));
    }

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            input.span(),
            "TypedUuid can only be derived for tuple structs",
        ));
    };
    let Fields::Unnamed(fields) = &data.fields else {
        return Err(syn::Error::new(
            data.fields.span(),
            "TypedUuid requires a tuple struct with one Uuid field",
        ));
    };
    if fields.unnamed.len() != 1 || !is_uuid(&fields.unnamed[0].ty) {
        return Err(syn::Error::new(
            fields.span(),
            "TypedUuid requires exactly one Uuid field",
        ));
    }

    Ok(quote::quote! {
        impl #struct_name {
            pub fn generate() -> Self {
                Self(::uuid::Uuid::now_v7())
            }

            pub const fn from_uuid(uuid: ::uuid::Uuid) -> Self {
                Self(uuid)
            }

            pub const fn as_uuid(&self) -> &::uuid::Uuid {
                &self.0
            }
        }

        impl ::std::convert::TryFrom<::std::string::String> for #struct_name {
            type Error = ::uuid::Error;

            fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, Self::Error> {
                ::uuid::Uuid::parse_str(&value).map(Self)
            }
        }

        impl ::std::fmt::Display for #struct_name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, formatter)
            }
        }
    })
}

/// tuple structの内包型がUuidか判定する。
fn is_uuid(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Uuid")
}

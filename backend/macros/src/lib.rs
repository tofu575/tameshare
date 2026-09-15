mod typed_string;
mod typed_uuid;
use proc_macro::TokenStream;

#[proc_macro_derive(TypedString, attributes(typed_string))]
/// String newtypeへ共通の文字数検証と定型trait実装を生成する。
pub fn derive_typed_string(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    typed_string::typed_string_macro_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(TypedUuid)]
/// UUID newtypeへUUID v7生成と定型trait実装を生成する。
pub fn derive_typed_uuid(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    typed_uuid::typed_uuid_macro_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

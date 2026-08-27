mod typed_string;
use proc_macro::TokenStream;

#[proc_macro_derive(NewFunction)]
pub fn derive_new_function(input: TokenStream) -> TokenStream {
    // 入力検証はここで済ませる
    // deriveマクロの最初の入力解析先はDeriveInputほぼ一択
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // マクロの本体を呼び出し
    typed_string::typed_string_macro_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

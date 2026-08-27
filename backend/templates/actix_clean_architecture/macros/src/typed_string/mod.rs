use syn::{Data, DeriveInput, spanned::Spanned};

pub fn typed_string_macro_impl(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;

    // 構造体であることを確認
    let fields = match input.data {
        Data::Struct(ref data_struct) => &data_struct.fields,
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "NewFunction can only be derived for structs",
            ));
        }
    };

    // フィールドの名前と型を取得
    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // new関数の生成
    let function = quote::quote! {
        impl #struct_name {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    Ok(function)
}

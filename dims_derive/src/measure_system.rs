use proc_macro::TokenStream;
use quote::quote;

pub fn impl_measure(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dims_core::MeasureSystem for #name {}
        impl std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                true
            }
        }
    };
    gen.into()
}

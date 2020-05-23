use proc_macro::TokenStream;
use quote::quote;

pub fn impl_measure(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dims_core::unit_creation::MeasureSystem for #name {}
        impl core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                true
            }
        }
        impl Clone for #name{
            fn clone(&self) -> Self {
                *self
            }
        }
        impl Copy for #name{}
    };
    gen.into()
}

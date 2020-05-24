use core::iter::{FromIterator, IntoIterator};
use dims_core::prelude::Flt;
use proc_macro::TokenStream;
use quote::quote;

const PREFIX_SIZE: usize = 2;
pub fn si_unit_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let unit = &ast.generics.type_params().next().unwrap().ident;
    let mut all_vals: [proc_macro::TokenStream; 2] =
        unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    let mut index = 0;
    for (val, pre) in &PREFIX_LIST {
        let gen = quote! {
            pub const #pre#name: dims_core::unit_creation::UnitSimple<#unit> =
            dims_core::unit_creation::UnitSimple<#unit> {
                system: dims_core::unit_creation::PhantomData,
                ratio: val,
                offset: 0.0, // TODO MAYBE?
            }
        };
        all_vals[1] = gen.into();
        index += 1;
    }

    let iter = all_vals.into_iter();
    // let token: TokenStream = TokenStream::from_iter(all_vals.into());
    // token.into()
    todo!()
}

const PREFIX_LIST: [(Flt, &str); PREFIX_SIZE] = [
    // Start out in the middle
    (0.1, "DECI"),
    (10.0, "DECA"),
];

mod measure_system;
use proc_macro::TokenStream;

#[proc_macro_derive(MeasureSystem)]
/// A macro to quickly mark the given struct as a "MeasureSystem"
///
/// This applies "MeasureSystem" and "PartialEq" (which always returns true; the value stored is important)
pub fn measure_system_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).expect("Unable to parse struct information!");

    // Build the trait implementation
    measure_system::impl_measure(&ast)
}

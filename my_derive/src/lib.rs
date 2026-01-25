use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ConfigTag)]
pub fn derive_my_tag(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let expanded = quote! {
        impl ConfigTag for #ident {}
    };

    TokenStream::from(expanded)
}

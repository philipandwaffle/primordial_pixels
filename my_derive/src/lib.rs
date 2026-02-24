use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ConfigTag)]
pub fn derive_my_tag(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let ident = input.ident;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ConfigTag for #ident #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

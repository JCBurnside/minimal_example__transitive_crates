use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature, parse_macro_input};



#[proc_macro_attribute]
pub fn custom_attribute(_meta:TokenStream,input :TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let Signature {
        ident,
        ..
    } = input.sig;
    TokenStream::from(quote! {
        struct #ident;
        impl ::supporting_crate::CustomTrait for #ident {
            fn name() -> &'static str {
                "foo"
            }
        }
    })
}
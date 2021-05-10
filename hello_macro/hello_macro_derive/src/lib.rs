extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Construit l'implementation du trait
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let nom = &ast.ident;
    let generation = quote! {
        impl HelloMacro for #nom {
            fn hello_macro() {
                println!("Hello Macro! mon nom est : {}", stringify!(#nom));
            }
        }
    };
    generation.into()
}

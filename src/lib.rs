#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitInt, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(Termination, attributes(exit_code))]
pub fn derive_termination(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match build_termination(&input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn build_termination(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let enum_name = &input.ident;
    let Data::Enum(enum_data) = &input.data else {
        return Err(syn::Error::new(
            input.span(),
            "Termination can only be derived for enum",
        ));
    };

    let mut variant_to_exit_code = Vec::new();
    for variant in &enum_data.variants {
        for attr in &variant.attrs {
            let path = attr.path();
            if path.is_ident("exit_code") {
                let code: LitInt = attr.parse_args()?;
                variant_to_exit_code.push((variant, code));
            }
        }
    }

    let arms: TokenStream2 = variant_to_exit_code
        .into_iter()
        .map(|(variant, code)| {
            let ident = &variant.ident;
            let variant = match variant.fields {
                Fields::Unit => quote! [ Self::#ident ],
                Fields::Unnamed(_) => quote! [ Self::#ident ( .. ) ],
                Fields::Named(_) => quote! [ Self::#ident { .. } ],
            };
            quote! {
                #variant => ::std::process::ExitCode::from(#code),
            }
        })
        .collect();

    Ok(quote! {
        impl ::std::process::Termination for #enum_name {
            fn report(self) -> ::std::process::ExitCode {
                match self {
                    #arms
                }
            }
        }
    })
}

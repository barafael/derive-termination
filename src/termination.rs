use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitInt, Variant};

pub fn build_termination_impl(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let enum_ident = &input.ident;
    let mappings = collect_variant_mappings(input, "Termination")?;

    let arms = mappings.iter().map(|(variant, code)| {
        let pattern = variant_pattern(variant);
        quote! { #pattern => ::std::process::ExitCode::from(#code), }
    });

    Ok(quote! {
        impl ::std::process::Termination for #enum_ident {
            fn report(self) -> ::std::process::ExitCode {
                match self {
                    #(#arms)*
                }
            }
        }
    })
}

pub fn build_exit_code_table_impl(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let enum_ident = &input.ident;
    let mappings = collect_variant_mappings(input, "ExitCodeTable")?;

    let entries = mappings.iter().map(|(variant, code)| {
        let name = variant.ident.to_string();
        quote! { (#code, #name) }
    });

    Ok(quote! {
        impl #enum_ident {
            /// Map from exit code to variant name.
            pub fn exit_code_to_variant() -> ::std::collections::BTreeMap<u8, &'static str> {
                ::std::collections::BTreeMap::from([#(#entries,)*])
            }
        }
    })
}

fn collect_variant_mappings<'a>(
    input: &'a DeriveInput,
    derive_name: &str,
) -> syn::Result<Vec<(&'a Variant, u8)>> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            format!("{derive_name} can only be derived for enums"),
        ));
    };

    data.variants
        .iter()
        .map(|variant| Ok((variant, parse_exit_code(variant)?)))
        .collect()
}

fn parse_exit_code(variant: &Variant) -> syn::Result<u8> {
    let Some(attr) = variant
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("exit_code"))
    else {
        return Err(syn::Error::new_spanned(
            variant,
            format!(
                "Variant '{}' requires an #[exit_code(N)] attribute",
                variant.ident
            ),
        ));
    };

    let lit: LitInt = attr.parse_args()?;
    lit.base10_parse()
}

fn variant_pattern(variant: &Variant) -> TokenStream2 {
    let ident = &variant.ident;
    match variant.fields {
        Fields::Unit => quote! { Self::#ident },
        Fields::Unnamed(_) => quote! { Self::#ident(..) },
        Fields::Named(_) => quote! { Self::#ident { .. } },
    }
}

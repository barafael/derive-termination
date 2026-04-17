#![doc = include_str!("../README.md")]

mod termination;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive `std::process::Termination` for an enum. See crate docs for details.
#[proc_macro_derive(Termination, attributes(exit_code))]
pub fn derive_termination(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    termination::build_termination_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derive an inherent `exit_code_to_variant()` method on an enum. See crate docs for details.
#[proc_macro_derive(ExitCodeTable, attributes(exit_code))]
pub fn derive_exit_code_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    termination::build_exit_code_table_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

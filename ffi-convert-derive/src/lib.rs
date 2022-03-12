//! This crate provides ffi_convert derive macros for CReprOf, AsRust and CDrop traits.

extern crate proc_macro;

mod asrust;
mod cdrop;
mod creprof;
mod rawpointerconverter;
mod utils;

mod asrust_enum;
mod cdrop_enum;
mod creprof_enum;
mod utils_enum;

use asrust::impl_asrust_macro;
use cdrop::impl_cdrop_macro;
use creprof::impl_creprof_macro;
use asrust_enum::impl_asrust_enum_macro;
use creprof_enum::impl_creprof_enum_macro;
use cdrop_enum::impl_cdrop_enum_macro;
use proc_macro::TokenStream;
use rawpointerconverter::impl_rawpointerconverter_macro;

#[proc_macro_derive(
    CReprOf,
    attributes(
        target_type,
        nullable,
        c_repr_of_convert,
        target_name
    )
)]
pub fn creprof_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_creprof_macro(&ast)
}

#[proc_macro_derive(
    CReprOfEnum,
    attributes(
        case,
        pointee,
        target_type,
        default
    )
)]
pub fn creprof_enum_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_creprof_enum_macro(&ast)
}

#[proc_macro_derive(
    AsRust,
    attributes(
        target_type,
        nullable,
        as_rust_extra_field,
        as_rust_ignore,
        target_name
    )
)]
pub fn asrust_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_asrust_macro(&ast)
}

#[proc_macro_derive(
    AsRustEnum,
    attributes(
        case,
        pointee,
        target_type
    )
)]
pub fn asrust_enum_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_asrust_enum_macro(&ast)
}

#[proc_macro_derive(CDrop, attributes(no_drop_impl, nullable))]
pub fn cdrop_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_cdrop_macro(&ast)
}

#[proc_macro_derive(
    CDropEnum,
    attributes(
        pointee,
    )
)]
pub fn cdrop_enum_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_cdrop_enum_macro(&ast)
}

#[proc_macro_derive(RawPointerConverter)]
pub fn rawpointerconverter_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_rawpointerconverter_macro(&ast)
}

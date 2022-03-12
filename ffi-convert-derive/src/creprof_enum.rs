use proc_macro::TokenStream;

use quote::quote;

use crate::utils_enum::{parse_enum_cases, Variant};
use crate::utils::parse_target_type;

pub fn impl_creprof_enum_macro(input: &syn::DeriveInput) -> TokenStream {
    let enum_name = &input.ident;
    let target_type = parse_target_type(&input.attrs);

    let enum_matches = parse_enum_cases(&input.data)
        .iter()
        .map(|case| {
            let Variant { name, case_name, pointee } = case;

            if let Some(pointee) = pointee {
                quote!(
                    #target_type::#case_name(v) => (#enum_name::#name, #pointee::c_repr_of(v)?.into_raw_pointer() as *const _)
                )
            } else {
                quote!(
                    #target_type::#case_name => (#enum_name::#name, std::ptr::null() as *const _)
                )
            }
        })
        .collect::<Vec<_>>();

    let c_repr_of_impl = quote!(
        impl CReprOfEnum<#target_type> for #enum_name {
            fn c_repr_of(input: #target_type) -> Result<(Self, *const libc::c_void), ffi_convert::CReprOfError> {
                // use ffi_convert::RawPointerConverter;
                Ok(match input {
                    #(#enum_matches, )*
                })
            }
        }
    );
    c_repr_of_impl.into()
}

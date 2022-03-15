use proc_macro::TokenStream;

use quote::quote;

use crate::utils_enum::{parse_enum_cases, Variant};
use crate::utils::parse_target_type;

pub fn impl_creprof_enum_macro(input: &syn::DeriveInput) -> TokenStream {
    let enum_name = &input.ident;
    let target_type = parse_target_type(&input.attrs);
    let mut default_variant: Option<Variant> = None;

    let enum_matches = parse_enum_cases(&input.data)
        .iter()
        .filter_map(|case| {
            if case.is_default {
                default_variant = Some((*case).clone());
                None
            } else {
                Some(case)
            }
        })
        .map(|case| {
            let Variant { name, pointee, .. } = case;
            let case_name = case.case_name.clone().expect("Non default variant should have a case name");

            if let Some(pointee) = pointee {
                let ty = pointee.ty.clone();

                if pointee.is_string {
                    quote!(
                       #target_type::#case_name(v) => (#enum_name::#name, std::ffi::CString::c_repr_of(v)?.into_raw_pointer() as *const _)
                    )
                } else {
                    quote!(
                       #target_type::#case_name(v) => (#enum_name::#name, #ty::c_repr_of(v)?.into_raw_pointer() as *const _)
                    )
                }
            } else {
                quote!(
                    #target_type::#case_name => (#enum_name::#name, std::ptr::null() as *const _)
                )
            }
        })
        .collect::<Vec<_>>();

    let c_repr_of_expr = if let Some(variant) = default_variant {
        let Variant { name, case_name, .. } = variant;

        quote!(
            Ok(match input {
                #(#enum_matches, )*
                _ => (#enum_name::#name, std::ptr::null() as *const _)
            })
        )
    } else {
        quote!(
            Ok(match input {
                #(#enum_matches, )*
            })
        )
    };

    let c_repr_of_impl = quote!(
        impl CReprOfEnum<#target_type> for #enum_name {
            fn c_repr_of(input: #target_type) -> Result<(Self, *const libc::c_void), ffi_convert::CReprOfError> {
                #c_repr_of_expr
            }
        }
    );
    c_repr_of_impl.into()
}

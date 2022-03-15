use proc_macro::TokenStream;
use quote::quote;

use crate::utils_enum::{parse_enum_cases, Variant};

pub fn impl_cdrop_enum_macro(input: &syn::DeriveInput) -> TokenStream {
    let enum_name = &input.ident;
    let mut has_default_branch = false;

    let enum_matches = parse_enum_cases(&input.data)
        .iter()
        .filter_map(|case| {
            if case.is_default {
                has_default_branch = true;
                None
            } else {
                Some(case)
            }
        })
        .map(|case| {
            let Variant { name, pointee, .. } = case;

            if let Some(pointee) = pointee {
                let ty = pointee.ty.clone();

                if pointee.is_string {
                    quote!(
                        #enum_name::#name => unsafe {std::ffi::CString::drop_raw_pointer(data as *const _)?}
                    )
                } else {
                    quote!(
                        #enum_name::#name => unsafe {#ty::drop_raw_pointer(data as *const _)?}
                    )
                }
            } else {
                quote!(
                    #enum_name::#name => {}
                )
            }
        })
        .collect::<Vec<_>>();

    let do_drop_expr = if has_default_branch {
        quote!(
            Ok(match self {
                #(#enum_matches, )*
                _ => ()
            })
        )
    } else {
        quote!(
            Ok(match self {
                #(#enum_matches, )*
            })
        )
    };

    quote!(
        impl CDropEnum for #enum_name {
            fn do_drop(&self, data: *const libc::c_void) -> Result<(), ffi_convert::CDropError> {
                #do_drop_expr
            }
        }
    )
        .into()
}

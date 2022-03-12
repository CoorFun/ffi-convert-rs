use proc_macro::TokenStream;

use quote::quote;

use crate::utils_enum::{parse_enum_cases, Variant};
use crate::utils::parse_target_type;

pub fn impl_asrust_enum_macro(input: &syn::DeriveInput) -> TokenStream {
    let enum_name = &input.ident;
    let target_type = parse_target_type(&input.attrs);

    let enum_matches = parse_enum_cases(&input.data)
        .iter()
        .map(|case| {
            if case.is_default {
                let Variant { name,.. } = case;
                quote!(
                    #enum_name::#name => Err(ffi_convert::AsRustError::UnknownEnumVariant)
                )
            } else {
                let Variant { name, pointee,.. } = case;
                let case_name = case.case_name.clone().expect("Non default variant should have a case name");

                if let Some(pointee) = pointee {
                    let conversion = quote!({
                    let ref_to_struct = unsafe { #pointee::raw_borrow(data as *const _)? };
                    let converted_struct = ref_to_struct.as_rust()?;
                    converted_struct
                });

                    quote!(
                    #enum_name::#name => Ok(#target_type::#case_name(#conversion))
                )
                } else {
                    quote!(
                    #enum_name::#name => Ok(#target_type::#case_name)
                )
                }
            }
        })
        .collect::<Vec<_>>();

    quote!(
        impl AsRustEnum<#target_type> for #enum_name {
            fn as_rust(&self, data: *const libc::c_void) -> Result<#target_type, ffi_convert::AsRustError> {
                match self {
                    #(#enum_matches, )*
                }
            }
        }
    )
    .into()
}

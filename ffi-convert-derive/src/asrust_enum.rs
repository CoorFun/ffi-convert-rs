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
            let Variant { name, case_name, pointee } = case;

            let conversion = quote!({
                let ref_to_struct = unsafe { #pointee::raw_borrow(data as *const _)? };
                let converted_struct = ref_to_struct.as_rust()?;
                converted_struct
            });

            quote!(
                #enum_name::#name => #target_type::#case_name(#conversion)
            )
        })
        .collect::<Vec<_>>();

    quote!(
        impl AsRustEnum<#target_type> for #enum_name {
            fn as_rust(&self, data: *const libc::c_void) -> Result<#target_type, ffi_convert::AsRustError> {
                Ok(match self {
                    #(#enum_matches, )*
                })
            }
        }
    )
    .into()
}

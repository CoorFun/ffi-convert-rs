use proc_macro::TokenStream;
use quote::quote;

use crate::utils_enum::{parse_enum_cases, Variant};

pub fn impl_cdrop_enum_macro(input: &syn::DeriveInput) -> TokenStream {
    let enum_name = &input.ident;

    let enum_matches = parse_enum_cases(&input.data)
        .iter()
        .map(|case| {
            let Variant { name, pointee, .. } = case;

            quote!(
                #enum_name::#name => unsafe {#pointee::drop_raw_pointer(data as *const _)?}
            )
        })
        .collect::<Vec<_>>();

    quote!(
        impl CDropEnum for #enum_name {
            fn do_drop(&self, data: *const libc::c_void) -> Result<(), ffi_convert::CDropError> {
                Ok(match self {
                    #(#enum_matches, )*
                })
            }
        }
    )
        .into()
}

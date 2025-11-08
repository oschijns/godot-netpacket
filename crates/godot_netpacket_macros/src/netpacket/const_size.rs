use super::{Error, encapsulate, to_size};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// Implement ConstSize derive
pub(crate) fn impl_derive_const_size(input: &DeriveInput) -> Result<TokenStream, Error> {
    let atype = &input.ident;
    match &input.data {
        // Implement for struct
        Data::Struct(astruct) => {
            let sizes = list_sizes(&astruct.fields);

            // write the implementation
            let tokens = quote![
                impl __net::ConstSize for #atype {
                    const SIZE: usize = #(#sizes)+*;
                }
            ];
            Ok(encapsulate(&tokens))
        }

        // Implement for enum without payload
        Data::Enum(anenum) => {
            // Check that the enum contains less than 256 variants.
            let count = anenum.variants.len();
            if count >= 256 {
                return Err(Error::TooManyVariants(count));
            }

            // Check that no variant of the enum contains a payload.
            for variant in &anenum.variants {
                if !variant.fields.is_empty() {
                    return Err(Error::NonConstSize);
                }
            }

            // write the implementation
            let tokens = quote! [
                impl __net::ConstSize for #atype {
                    const SIZE: usize = 1;
                }
            ];
            Ok(encapsulate(&tokens))
        }
        _ => Err(Error::Union),
    }
}

/// Create an array with an expression to evaluate the size of each field.
fn list_sizes(fields: &Fields) -> Vec<TokenStream> {
    let mut sizes = Vec::with_capacity(fields.len());

    // for each field get its static size
    for field in fields {
        sizes.push(to_size(field));
    }

    sizes
}

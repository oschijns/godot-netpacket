use super::{Error, encapsulate, write_offsets, write_structuring};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields};

/// Implement Deserialize derive
pub(crate) fn impl_derive_deserialize(input: &DeriveInput) -> Result<TokenStream, Error> {
    let atype = &input.ident;
    match &input.data {
        // Implement for struct
        Data::Struct(astruct) => {
            // Evaluate tokens
            let vars = write_structuring(&astruct.fields);
            let offsets = write_offsets(&astruct.fields);
            let deserials = write_deserial(&astruct.fields);

            // write the implementation
            let tokens = quote! [
                impl __net::Deserialize for #atype {
                    fn deserialize(
                        __buffer: &__godot::PackedByteArray,
                        __offset: usize
                    ) -> core::result::Result<Self, ()>
                        where
                            Self: core::marker::Sized
                    {
                        const __OFFSET_0: usize = 0;
                        #(#offsets)*
                        #(#deserials)*

                        core::result::Result::Ok(Self { #(#vars),* })
                    }
                }
            ];
            Ok(encapsulate(&tokens))
        }

        // Implement for enum
        Data::Enum(anenum) => {
            let mut entries = Vec::with_capacity(anenum.variants.len());

            // Implement serialization for each variant
            // Just use the raw index of the variant to identify it.
            // TODO: Use the discriminant as the identifier.
            for (index, variant) in anenum.variants.iter().enumerate() {
                // Evaluate tokens
                let index = index as u8;
                let name = &variant.ident;
                let vars = write_structuring(&variant.fields);
                let offsets = write_offsets(&variant.fields);
                let deserials = write_deserial(&variant.fields);

                entries.push(quote! [
                    #index => {
                        #(#offsets)*
                        #(#deserials)*

                        core::result::Result::Ok(Self::#name { #(#vars),* })
                    }
                ]);
            }

            // write the implementation
            let tokens = quote! [
                impl __net::Deserialize for #atype {
                    fn deserialize(
                        __buffer: &__godot::PackedByteArray,
                        __offset: usize
                    ) -> core::result::Result<Self, ()>
                        where
                            Self: core::marker::Sized
                    {
                        let id = __buffer.decode_u8(__offset)?;
                        const __OFFSET_0: usize = 1;

                        match id {
                            #(#entries),*

                            _ => core::result::Result::Err(()),
                        }
                    }
                }
            ];
            Ok(encapsulate(&tokens))
        }
        _ => Err(Error::Union),
    }
}

/// Call deserialize for each field
fn write_deserial(fields: &Fields) -> Vec<TokenStream> {
    // Store the generated tokens in this list
    let mut entries = Vec::with_capacity(fields.len());

    // for each field get its static size
    for (index, field) in fields.iter().enumerate() {
        // prepare tokens for the macro
        let atype = &field.ty;
        let name = format_ident!("__field_{}", index);
        let offset = format_ident!("__OFFSET_{}", index);

        // add a new entry to serialize
        entries.push(quote! [
            let #name = <#atype as __net::Deserialize>::deserialize(__buffer, __offset + #offset)?;
        ]);
    }

    entries
}

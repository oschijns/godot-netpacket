use super::{Error, encapsulate, write_offsets, write_structuring};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields};

/// Implement Serialize derive
pub(crate) fn impl_derive_serialize(input: &DeriveInput) -> Result<TokenStream, Error> {
    let atype = &input.ident;
    match &input.data {
        // Implement for struct
        Data::Struct(astruct) => {
            // Evaluate tokens
            let vars = write_structuring(&astruct.fields);
            let offsets = write_offsets(&astruct.fields);
            let serials = write_serial(&astruct.fields);

            // write the implementation
            let tokens = quote! [
                impl __net::Serialize for #atype {
                    fn serialize(
                        &self,
                        __buffer: &mut __godot::PackedByteArray,
                        __offset: usize
                    ) -> core::result::Result<(), ()> {
                        let Self { #(#vars),* } = self;

                        const __OFFSET_0: usize = 0;
                        #(#offsets)*
                        #(#serials)*

                        core::result::Result::Ok(())
                    }
                }
            ];
            Ok(encapsulate(&tokens))
        }

        // Implement for enum
        Data::Enum(anenum) => {
            let mut entries = Vec::with_capacity(anenum.variants.len());

            // Implement serialization for each variant.
            // Just use the raw index of the variant to identify it.
            // TODO: Use the discriminant as the identifier.
            for (index, variant) in anenum.variants.iter().enumerate() {
                // Evaluate tokens
                let index = index as u8;
                let name = &variant.ident;
                let vars = write_structuring(&variant.fields);
                let offsets = write_offsets(&variant.fields);
                let serials = write_serial(&variant.fields);

                entries.push(quote! [
                    Self::#name { #(#vars),* } => {
                        __buffer.encode_u8(__offset, #index)?;

                        #(#offsets)*
                        #(#serials)*
                    }
                ]);
            }

            // write the implementation
            let tokens = quote! [
                impl __net::Serialize for #atype {
                    fn serialize(
                        &self,
                        __buffer: &mut __godot::PackedByteArray,
                        __offset: usize
                    ) -> core::result::Result<(), ()> {
                        const __OFFSET_0: usize = 1;

                        match self {
                            #(#entries)*,
                        }

                        core::result::Result::Ok(())
                    }
                }
            ];
            Ok(encapsulate(&tokens))
        }
        _ => Err(Error::Union),
    }
}

/// Call serialize for each field
fn write_serial(fields: &Fields) -> Vec<TokenStream> {
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
            <#atype as __net::Serialize>::serialize(#name, __buffer, __offset + #offset)?;
        ]);
    }

    entries
}

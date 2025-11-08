//! Implement traits related to network packets

/// Implement derive macro for ConstSize trait
pub mod const_size;

/// Implement derive macro for Serialize trait
pub mod serialize;

/// Implement derive macro for Deserialize trait
pub mod deserialize;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Field, Fields};

/// Error encountered when implementing traits
#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Union types are not supported")]
    Union,

    #[error("Too many variants {0} in the provided enum")]
    TooManyVariants(usize),

    #[error("Cannot deduce constant size of enum with payload")]
    NonConstSize,
}

/// Convert the error into a syn::Error
impl From<Error> for syn::Error {
    fn from(value: Error) -> Self {
        syn::Error::new_spanned(TokenStream::new(), value.to_string())
    }
}

/// Generate tokens for structuring or destructuring a set of fields
fn write_structuring(fields: &Fields) -> Vec<TokenStream> {
    // Store the generated tokens in this list
    let mut entries = Vec::with_capacity(fields.len());

    // for each field, write a `real_name: __field_x` mapping
    for (index, field) in fields.iter().enumerate() {
        // prepare tokens
        let real_name = if let Some(ident) = &field.ident {
            ident.to_token_stream()
        } else {
            quote! [#index]
        };
        let tmp_name = format_ident!("__field_{}", index);

        // add a new mapping to the list
        entries.push(quote! [
            #real_name: #tmp_name
        ]);
    }

    entries
}

/// Generate a list of offsets for each of the fields.
fn write_offsets(fields: &Fields) -> Vec<TokenStream> {
    let mut offsets = Vec::with_capacity(fields.len());

    // for each field get its static size
    for (index, field) in fields.iter().enumerate() {
        // prepare tokens for the macro
        let size = to_size(field);
        let offset_prev = format_ident!("__OFFSET_{}", index);
        let offset_curr = format_ident!("__OFFSET_{}", index + 1);

        // Create a new const declaration to compute the offset of a field
        offsets.push(quote! [
            const #offset_curr: usize = #offset_prev + #size;
        ]);
    }

    offsets
}

/// Given a field get its static size as tokens
fn to_size(field: &Field) -> TokenStream {
    let atype = &field.ty;
    quote! [ <#atype as __net::ConstSize>::SIZE ]
}

/// Encapsulate the token stream into a scope with the necessary modules
fn encapsulate(tokens: &TokenStream) -> TokenStream {
    quote![
        const _: () = {
            use godot_netpacket as __net;
            use godot::builtin as __godot;

            #tokens
        };
    ]
}

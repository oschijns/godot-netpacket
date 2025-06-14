/// Implement derive macros to serialize and deserialize structs
mod netpacket;

use netpacket::{
    const_size::impl_derive_const_size, deserialize::impl_derive_deserialize,
    serialize::impl_derive_serialize,
};
use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

/// Derive macro to evaluate the static size of a serialized struct in bytes
#[proc_macro_derive(ConstSize)]
pub fn derive_const_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_derive_const_size(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => Error::from(err).to_compile_error().into(),
    }
}

/// Derive macro to serialize a struct into a packet
#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_derive_serialize(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => Error::from(err).to_compile_error().into(),
    }
}

/// Derive macro to deserialize a struct from packet
#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_derive_deserialize(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => Error::from(err).to_compile_error().into(),
    }
}

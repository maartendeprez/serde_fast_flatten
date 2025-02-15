extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod deserialize;
mod serialize;

#[proc_macro_derive(SerializeFields, attributes(serde))]
pub fn derive_serialize_fields(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);
    let tokens = serialize::serialize_fields(input);
    // eprintln!(
    //     "{}",
    //     prettyplease::unparse(&syn::parse_file(&tokens.to_string()).unwrap())
    // );
    TokenStream::from(tokens)
}

#[proc_macro_derive(DeserializeFields, attributes(serde))]
pub fn derive_deserialize_fields(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);
    let tokens = deserialize::deserialize_fields(input);
    // eprintln!(
    //     "{}",
    //     prettyplease::unparse(&syn::parse_file(&tokens.to_string()).unwrap())
    // );
    TokenStream::from(tokens)
}

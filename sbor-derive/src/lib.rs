mod categorize;
mod decode;
mod describe;
mod encode;
mod utils;

use proc_macro::TokenStream;

/// Derive code that returns the value kind.
#[proc_macro_derive(Categorize, attributes(sbor))]
pub fn categorize(input: TokenStream) -> TokenStream {
    categorize::handle_categorize(proc_macro2::TokenStream::from(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive code that encodes this data structure
#[proc_macro_derive(Encode, attributes(sbor))]
pub fn encode(input: TokenStream) -> TokenStream {
    encode::handle_encode(proc_macro2::TokenStream::from(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive code that decodes this data structure from a byte array.
#[proc_macro_derive(Decode, attributes(sbor))]
pub fn decode(input: TokenStream) -> TokenStream {
    decode::handle_decode(proc_macro2::TokenStream::from(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive code that describes the SBOR type.
#[proc_macro_derive(Describe, attributes(sbor))]
pub fn describe(input: TokenStream) -> TokenStream {
    describe::handle_describe(proc_macro2::TokenStream::from(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

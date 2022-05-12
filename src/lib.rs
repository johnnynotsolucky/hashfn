//! A procedural macro to generate a hash representation of a function as a string slice
//!
//! The hash is generated as a const with the same visibility as the function the macro is applied
//! to.
//!
//! ## Example:
//!
//! ```
//! use hashfn::hashfn;
//!
//! #[hashfn]
//! pub(crate) fn do_something() {}
//!
//! // Will expand to
//! // pub(crate) const DO_SOMETHING_HASH: &str = "<hash>";
//! // pub(crate) fn do_something() {}
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use sha2::{Digest, Sha256};
use core::fmt;
use std::fmt::Write;
use syn::{self, parse_macro_input, ItemFn};

#[allow(clippy::missing_panics_doc)]
#[proc_macro_attribute]
pub fn hashfn(_args: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    let function_name = function.sig.ident.to_string();

    let mut hasher = Sha256::new();
    hasher.update(format!("{}", quote!(#function)));
    let hash = hasher.finalize();

    let mut hex = String::with_capacity(2 * hash.len());
    if let Err(fmt::Error) = write!(hex, "{:x}", hash) {
        // What would make this actually fail?
        panic!("Unable to generate hash for \"{}\"", function_name);
    }

    let visibility = function.vis.clone();
    let const_name = format_ident!("{}_HASH", function_name.to_uppercase());

    let expanded = quote! {
        #visibility const #const_name: &str = #hex;

        #function
    };

    expanded.into()
}

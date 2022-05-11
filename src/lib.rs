use proc_macro::TokenStream;
use quote::{format_ident, quote};
use sha2::{Digest, Sha256};
use std::fmt::Write;
use syn::{self, parse_macro_input, ItemFn};

/// Generates a constant matching the visibility of the function holding a hash of that function
///
/// ## Example:
///
/// ```
/// use hashfn::hashfn;
///
/// #[hashfn]
/// pub(crate) fn do_something() {}
/// ```
///
/// expands to
///
/// ```
/// pub(crate) const DO_SOMETHING_HASH: &str = "<hash>";
/// pub(crate) fn do_something() {}
/// ```
#[proc_macro_attribute]
pub fn hashfn(_attribtues: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);

    let mut hasher = Sha256::new();
    hasher.update(format!("{}", quote!(#function)));
    let hash = hasher.finalize();

    let mut hex = String::with_capacity(2 * hash.len());
    for b in hash {
        let _ = write!(hex, "{:x}", b);
    }

    let visibility = function.vis.clone();
    let const_name = format_ident!("{}_HASH", function.sig.ident.to_string().to_uppercase());

    let expanded = quote! {
        #visibility const #const_name: &str = #hex;

        #function
    };

    expanded.into()
}

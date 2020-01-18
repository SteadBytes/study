// Comes with Rust, provides API for reading and manipulating Rust source code
extern crate proc_macro;

use crate::proc_macro::TokenStream;
// Turns `syn` data structures back into Rust code
use quote::quote;
// Parses Rust code from a string into a data structure
use syn;

// Call this macro whenever a uses specifies `#[derive(HelloMacro)]`
#[proc_macro_derive(HelloMacro)]
/// Implement `HelloMacro` for a given type.
/// ```
/// #[derive(HelloMacro)]
/// struct Pancakes;
/// ```
/// Turns into:
/// ```
/// #[derive(HelloMacro)]
/// struct Pancakes;
///
/// impl HelloMacro for Pancakes {
///   fn hello_macro() {
///     println!("Hello, Macro! My name is {}", "Pancakes");
///   }
/// }
/// ```
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree for manipulation
    // Panic on parse failure as procedural macros *must* return `TokenStream`
    // -> cannot return `Restult`
    // Note: Specify error messages using `panic!` or `expect` in real code
    // instead of `unwrap()`
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

/// Generate code implementing `HelloMacro` for an annotated type.
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // `stringify!` is built into rust and takes an expression at
                // compile time and turns it into a string literal
                // `stringify!(1 + 2)` -> `"1 + 2"
                // Necessary as `#name` could be an expression to print literally
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into() // Convert result of `quote!` macro to `TokenStream`
}

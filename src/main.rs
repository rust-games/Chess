//! chess game executable
// Careful to the syntax:
//
// | Documentation | Inner              | Outer              |
// |---------------|:------------------:|:------------------:|
// | Line          | //! doc comment    | /// doc comment    |
// | Block         | /*! doc comment */ | /** doc comment */ |
//
// - Inner attribute: #![allow(missing_docs)] (of the entire file)
// - Outer attribute: #[test] (consider the function bellow as a test)

// Good practice: use these attributes
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod chess;

/// This function returns the greeting: `Hello, world!`
fn main() {
    chess::lib_hello();
}

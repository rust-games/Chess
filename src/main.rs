//! chess game executable

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

use env_logger;

use chess::{run, ChessGui};

fn main() {
    // Init the logger
    env_logger::init();

    // Create and run the game
    run(ChessGui::default());
}

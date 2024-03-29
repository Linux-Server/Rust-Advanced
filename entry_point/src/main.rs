#![doc(html_logo_url = "https://rustacean.net/assets/cuddlyferris.svg")]
//! # Entry Point
//! This entry point binary crate act as the base crate , which have the ability to call an the other lib crate mentioned in the workspace
//! ## Examples
//! ```
//! fn main() {
//!     println!("Hello, Entry Point!");
//! }
//! ```

use closure::{declare_closure, type_inference, capture_reference};
fn main() {
    println!("Hello, Entry Point!");
    // declare_closure();
    // type_inference()
    capture_reference();

    let x = 10;
    let y = vec![1,2,3];
}

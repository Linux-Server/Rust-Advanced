#![doc(html_logo_url = "https://rustacean.net/assets/cuddlyferris.svg")]
//! # Entry Point
//! This entry point binary crate act as the base crate , which have the ability to call an the other lib crate mentioned in the workspace
//! ## Examples
//! ```
//! fn main() {
//!     println!("Hello, Entry Point!");
//! }
//! ```

use closure::declare_closure;
fn main() {
    println!("Hello, Entry Point!");
    declare_closure();
}

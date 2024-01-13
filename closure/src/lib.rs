//! # Closure
//! A function like construct, that you can store in a variable
//!  - Its an anonymous function that can capture their environment
//!  - closure can be save in a variable
//!  - pass args like other functions
//!  - can define closure in one place and call it in another context
//!  - Unlike functions,  **closure can capture values from the scope in which they are defined**
//!

pub fn declare_closure() {
    //! ## Declare a closure
    //!  closure can be declared with the following examples
    //! ## Examples
    //! ```
    //!    // Define an empty closure
    //!     let first_closure = || 10;
    //!     println!("The first closure : {:?}", first_closure());
    //!     // A closure receiving args
    //!     let x = |val| val+10;
    //!     println!("The first closure : {:?}", x(1));
    //!     // closure capturing env
    //!     let a = 10;
    //!     let capture_env = || println!("captured value: {:?}", a);
    //!     capture_env();
    //! ```
    //!

    // Define an empty closure
    let first_closure = || 10;
    println!("The first closure : {:?}", first_closure());
    // A closure receiving args
    let x = |val| val+10;
    println!("The first closure : {:?}", x(1));
    // closure capturing env
    let a = 10;
    let capture_env = || println!("captured value: {:?}", a);
    capture_env();



}

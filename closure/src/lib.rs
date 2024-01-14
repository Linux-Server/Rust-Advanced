#![doc(html_playground_url = "https://play.rust-lang.org")]
//! # Closure
//! A function like construct, that you can store in a variable
//!  - Its an anonymous function that can capture their environment
//!  - closure can be save in a variable
//!  - pass as args to other functions
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
    let x = |val| val + 10;
    println!("The first closure : {:?}", x(1));
    // closure capturing env
    let a = 10;
    let capture_env = || println!("captured value: {:?}", a);
    capture_env();
}

pub fn type_inference() {
    //! # Type inference
    //! - to give closure more strictness and readability
    //! - closure are not exposed explicitly to user, so type inference is not mandatory
    //! - Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).
    //!
    //! ```
    //!     let infers = |x: i32| x + 100;
    //!     println!("{}", infers(10));
    //!     let infer_return = |x:i32,y:i32|-> i32{
    //!        x+y
    //!     };
    //!     println!("{}", infer_return(10,20));
    //!
    //!     // The first time we call example_closure with the String value,
    //!     // the compiler infers the type of x and the return type of the closure to be String.
    //!     // Those types are then locked into the closure in example_closure,
    //!     // and we get a type error when we next try to use a different type with the same closure.
    //!
    //!     let example_closure = |x| x;
    //!     example_closure("sam");
    //!     // example_closure(10); // won't work
    //! ```
    let infers = |x: i32| x + 100;
    println!("{}", infers(10));

    let infer_return = |x:i32,y:i32|-> i32{
        x+y
    };
    println!("{}", infer_return(10,20));

    let example_closure = |x| x;
    example_closure("sam");
    // example_closure(10); // won't work


}

pub fn capture_reference(){
    //! # Capture reference
    let list = vec![1,2,3,4,5];
    let only_borrow = || println!("{:?}", list);
    only_borrow(); // call the closure
    println!("The list after borrow : {:?}", list);
}

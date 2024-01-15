#![doc = include_str!("docs/closure.md")]

pub fn declare_closure() {
    #![doc = include_str!("docs/declare_closure.md")]
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
    #![doc = include_str!("docs/type_inferance.md")]
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
    //! Closures can capture values from their environment in three ways,
    //! which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership.
    //! The closure will decide which of these to use based on what the body of the function does with the captured values.
    //!
    //! ## Borrow immutably
    //!
    //! ```
    //!     let list = vec![1,2,3,4,5];
    //!     let only_borrow = || println!("{:?}", list);
    //!     only_borrow(); // call the closure
    //!     println!("The list after borrow : {:?}", list);
    //! ```
    //!


    let list = vec![1,2,3,4,5];
    let v2 = vec![1,2,3];
    let only_borrow = || println!("{:?}", list);
    only_borrow(); // call the closure
    println!("The list after borrow : {:?}", list);

    let mut list_2 = vec![1,2,3,4,5,6];
    let mut borrow_mut = || list_2.push(100);
    borrow_mut();
    println!("The list_2 after mut borrow : {:?}", list_2);

}


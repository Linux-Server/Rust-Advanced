 ## Declare a closure
  closure can be declared with the following examples
 ## Examples
 ```
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
 ```

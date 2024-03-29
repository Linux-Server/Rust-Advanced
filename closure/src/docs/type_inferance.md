 ## Type inference
 - to give closure more strictness and readability
 - closure are not exposed explicitly to user, so type inference is not mandatory
 - Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

 ## Examples

 ```
     let infers = |x: i32| x + 100;
     println!("{}", infers(10));
     let infer_return = |x:i32,y:i32|-> i32{
        x+y
     };
     println!("{}", infer_return(10,20));

     // The first time we call example_closure with the String value,
     // the compiler infers the type of x and the return type of the closure to be String.
     // Those types are then locked into the closure in example_closure,
     // and we get a type error when we next try to use a different type with the same closure.

     let example_closure = |x| x;
     example_closure("sam");
     // example_closure(10); // won't work
 ```
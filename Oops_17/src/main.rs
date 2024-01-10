/// The game starts from here a.k.a main()

//Object-Oriented Programming Features of Rust

/*
 - oops is a paradigm in modelling programs
 - here we discuss more of benefits and tradeoff of using oops concept in rust
 - rust is influenced by many paradigm, once of such is oops
 -  examples of programming paradigm , oops, procedural, functional etc..
 - The features that rust acquired from oops are objects,encapsulation,inheritance
 -

 */
pub mod char_17_1;
pub mod animal;

// use char_17_1::{encapsulation,inheritance,objects};
fn main() {
    //! inner main

    println!("Advanced OOPS!");

}

    pub struct Person{
        // This the name of person
        pub name:String,
        /// This is age of the person
        pub age: i32
    }






/* Stuation 3 
--------------
Treating Smart Pointers Like Regular References with the Deref Trait

Deref trait allows you to customize the behavior of the dereference operator *
content:
     1) how the dereference operator works with regular references.
     2) Using Box<T> instead of a reference
     3) Defining our own smart pointer 
     4) Treating a Type Like a Reference by Implementing the Deref Trait
     5) Implicit Deref Coercions with Functions and Methods
     6)How Deref Coercion Interacts with Mutability


*/

use std::ops::Deref;

/*
 1) how the dereference operator works with regular references.*/
 pub fn derefop(){
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y) //it's an error. if we want assertion about the value in y, we have to use *y
 } 
/* 
2) Using Box<T> instead of a reference */
pub fn boxop() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);//use dereference operator for box same as refences
}

/*
3) Defining our own smart pointer */


struct MyBox1<T>(T);

impl<T> MyBox1<T> {//generic so it can hold any type
    fn new(x: T) -> MyBox1<T> {
        MyBox1(x)
    }
}

pub fn ud_sp(){
    let x = 5;
    let y = MyBox1::new(x);

    assert_eq!(5, x);
    // println!("user defined smart pointer {}",*y);
    // assert_eq!(5, *y);//its an error user defined smart pointer cant deref
}

/*
4) Treating a Type Like a Reference by Implementing the Deref Trait

Deref trait, provided by the standard library, 
requires us to implement one method named deref that borrows self and returns a reference to the inner data.

without deref trait compiler can only  dereference & references.

with deref trait compiler has the ability to take a value of any type that implements Deref 
by call the deref method to get a & reference that it knows how to dereference.
eg: *(y.deref())

*/

struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}

impl<T> MyBox<T> {//generic so it can hold any type
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

pub fn ud_sp_deref(){
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    println!("with deref trait user defined smart pointer {}",*y);
    assert_eq!(5, *(y.deref()));
    assert_eq!(5, *y);

}

/*
5) Implicit Deref Coercions with Functions and Methods

Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
eg: deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.
*/

pub fn coercion(name: &str) {
    println!("Hello, {name}!");
}

pub fn noo_coercion(){
        let m = MyBox::new(String::from("Rust"));
        coercion(&(m)[..]);

}


/*
6)How Deref Coercion Interacts with Mutability

you can use the DerefMut trait to override the * operator on mutable references.

deref coercion when it finds types and trait implementations in three cases:

From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
*/
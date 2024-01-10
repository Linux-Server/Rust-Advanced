/* Box<T>
------
Boxes store data on the heap and pointer to its heap data stores on stack.
Boxes provide only the indirection and heap allocation

Box  don’t have many extra capabilities .most often used situations:
      1> when hou have a type with unknown size at compile time and need to use it (at compile time size of each type should be known)
      2> when you want to transfer the ownership of large amount of data without copying its content
      3> When you want to own a value and only care that it implement a specific trait
  */

use crate::box_15_1::List::{Cons,Nil};
/*
1) create box and store i32 data */
pub fn box_create() {
    let b = Box::new(5);//5 store in the heap and pointer store stack
    println!("b = {}", b);
}// o/p-> b=5


/*
2) situation 1 and 2:
----------------------
 At compile time size of each type should be known.
 Nesting of values of recursive types could theoretically continue infinitely.
 Boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.
 
 enum List {
    Cons(i32, List),
    Nil,
}
this type has infinite size

 */


#[derive(Debug)]
 pub enum List {
  Cons(i32, Box<List>),
  Nil,
}

pub fn enable_recursive() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("{:?}",list)
}



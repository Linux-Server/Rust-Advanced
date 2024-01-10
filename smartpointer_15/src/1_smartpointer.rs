/*
A pointer is a general concept for a variable that contains an address in memory. 
on other hand smart pointer contain additional metadata and extra capabilities or guarantees.

diff b/w smart pointer and reference: references only borrow data, smart pointers own the data they point to.

Smart pointers are usually implemented using structs. 
Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. 

Deref trait --> allow  smart pointer struct to behave like a reference 
Drop trait--> allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

smart pointers
--------------
1. String
2. Vector
3. Box<T> for allocating values on the heap
4. Rc<T>, a reference counting type that enables multiple ownership
5. Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

 */
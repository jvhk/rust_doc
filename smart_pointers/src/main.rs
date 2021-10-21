/*
    SMART POINTERS

    Box<T> for allocating values on the heap
    Rc<T>, a reference counting type that enables multiple ownership
    Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time


*/

enum List{
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons,Nil};

//Defining Our Own Smart Pointer
struct  MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}


// Implementing Deref with deref method to MyBox struct
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; //The type Target = T; syntax defines an associated type for the Deref trait to use

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

fn hello(name: &str){
    println!("Hello {} !", name)
}

fn main() {
    let boxFive = Box::new(5);
    println!("box five {}", boxFive);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    //Treating Smart Pointers Like Regular References with the Deref Trait
    let x = 5;
    let y = &x;

    assert_eq!(5,x);
    assert_eq!(5, *y);

    println!("{}", y);

    //Using Box<T> Like a Reference
    let a = 5;
    let b = Box::new(a);

    assert_eq!(5,a);
    assert_eq!(5, *b);

    println!("{}", b);

    //Defining Our Own Smart Pointer
    let c = 5;
    let d = MyBox::new(a);

    assert_eq!(5,c);
    assert_eq!(5, *d); // -> returns: cannot be dereferenced (error) (needs to implement Deref to the struct)

    // Implicit Deref Coercions with Functions and Methods
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

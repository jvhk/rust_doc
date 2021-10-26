/*
    SMART POINTERS

    Box<T> for allocating values on the heap
    Rc<T>, a reference counting type that enables multiple ownership
    Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time


*/
#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
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


// Running code on clean up with the Drop Trair
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`", self.data)
    }
}

// Using Rc<T> to Share Data
enum RcList{
    Cons(i32, Rc<List>),
    Nil,
}

// use crate::RcList::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let boxFive = Box::new(5);
    println!("box five {}", boxFive);

    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

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
    hello(&(*m)[..]);


    //// Running code on clean up with the Drop Trair
    println!("Running code on clean up with the Drop Trair:");
    let custom = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let custom2 = CustomSmartPointer{
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");

    //Dropping a Value Early with std::mem::drop (not permited)
    println!("Dropping a Value Early with std::mem::drop");
    let custom_to_drop = CustomSmartPointer{
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created to std::mem::drop");
    // custom_to_drop.drop(); //returns a error, because this method cannot be called directely
    std::mem::drop(custom_to_drop);
    println!("CustomSmartPointer dropped before the and of `main`");

    
    // Using Rc<T> to Share Data

    /*    
    let f = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating f = {}", Rc::strong_count(&f));
    let g = Cons(3, Rc::clone(&f));
    println!("count after creating g = {}", Rc::strong_count(&f));
    {
        let h = Cons(4, Rc::clone(&f));
        println!("count after creating h = {}", Rc::strong_count(&f));
    }
    println!("count after h goes out of scope = {}", Rc::strong_count(&f));
    */


    //Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    let value = Rc::new(RefCell::new(5));

    let e = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))) ;

    let f = Cons(Rc::new(RefCell::new(3)), Rc::clone(&e));
    let g = Cons(Rc::new(RefCell::new(4)), Rc::clone(&e));

    *value.borrow_mut() += 10;

    println!("e after = {:?}", e);
    println!("f after = {:?}", f);
    println!("g after = {:?}", g);
}

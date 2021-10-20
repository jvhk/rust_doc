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

fn main() {
    let boxFive = Box::new(5);
    println!("box five {}", boxFive);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

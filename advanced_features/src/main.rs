use std::{ops::Add, slice};

//calling an unsafe func or method
unsafe fn dangerous(){}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}


//Using extern Functions to Call External Code
extern "C"{
    fn abs(input:i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32){
    unsafe{
        COUNTER += inc;
    }
}

//Accessing or Modifying a Mutable Static Variable
static HELLO_WORLD: &str = "Hello world!";

fn main() {
   //Dereferencing a Raw Pointer
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
        
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    let address = 0x012345usize;
    let r = address as *const i32;


    //calling an unsafe func or method
    unsafe {
        dangerous();
    }
 


    let mut v = vec![1,2,3,4,5,6];
    let mutvect = &mut v[..];

    let (a,b) = mutvect.split_at_mut(3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
    
    //Using extern Functions to Call External Code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    
    println!("name is: {}, from static variable", HELLO_WORLD);

    add_to_count(3);

    unsafe{
        println!("COUNTER: {}", COUNTER);
    }
}

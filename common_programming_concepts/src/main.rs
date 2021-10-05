use std::io;

fn main(){
    /*
    let guess: u32 = "42".parse().expect("not a number");


    //tuples
    let tup = (10, 20, 30);
    let (x,y,z) = tup;
    println!("value of y {}", y); //20

    //array

    let array: (i32, f64, u8) = (500, 6.3, 1);
    let array2 = [1,2,3,4,5];
    
    let first_of_array = array.0;
    let last_of_array = array.2;
    println!("first array item {}, last array item {}", first_of_array, last_of_array);
    */


    // >> Invalid Array Element Access

    let a = [1,2,3,4,5];

    println!("please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    // >> FUNCTIONS
    another_function();
    another_function_with_params(20);
    let returned = function_with_return(); //5
    println!("function_with_return {}", returned);
    let returned_with_params = function_with_return_and_params(5); //10
    println!("function_with_return_and_params {}", returned_with_params);

    fn another_function(){
        println!("another function scope");
    }

    fn another_function_with_params(x: i32){
        println!("parameter function number {}", x);
    }

    fn function_with_return() -> i32{
        5
    } 
    
    fn function_with_return_and_params(x: i32) -> i32{
        x + 5
    }
}
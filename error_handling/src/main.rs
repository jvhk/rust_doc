use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

enum Result<T,E> {
    Ok(T),
    Err(E),
}

fn panic_func(){
    println!("here is executable!");
    // !panic macro
    panic!("crash and burn");

    println!("here codes do not execute");
}

fn backtrace_panic(){
    let v = vec![1,2,3];

    v[99];
}

fn read_file_that_not_exists(){
    let f= File::open("hello.txt");

    let f = match f {
        Ok(f) => f,
        Err(error) => panic!("Problem opening the file : {:?}", error),
    };
}

fn matching_different_errors(){
    let f = File::open("not.txt");

    let f = match f {
        Ok(f) => f,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("not.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating {:?} file", err),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

//reading a username from file
/*
fn propagating_errors() ->  Result<String, io::Error> { 
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
} 
 */



fn main(){
    //EXAMPLES: panic! and backtrace_panic
    // panic_func();
    // backtrace_panic(); // returns: thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99

    // EXAMPLES: handling Result errors
    // read_file_that_not_exists();
    //matching_different_errors();
}
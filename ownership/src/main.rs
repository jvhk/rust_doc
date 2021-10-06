
// >> Scopes

/*
fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    let x = 5;
    
    makes_copy(x);
}


fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
*/

// >> References and Borrowing

/*
fn main() {

    //let s1 = String::from("hello");

    //let len = calculate_length(&s1);

    //println!("The length of '{}' is {}.", s1, len);
    
    //let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    println!("just &s : {}", &s);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

*/

// >> The Slice Type

fn main(){
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str{
    let bytes= s.as_bytes();

    for (i, &item) in  bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}
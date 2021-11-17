
//function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {

    // NORMAL PATTERN MATCHING
    /*match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }*/ 


    //Conditional if let Expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color{
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday{
        println!("Tuesday is green day");
    } else if let Ok(age) = age{
        if age > 30{
            println!("Using purple as the background color");
        } else{
            println!("Using orange as the background color");
        }
    } else {
        println!("Using orange as the background color");
    }


    // while let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //for Loops
    let vect = vec!['a','b','c'];
    for (index,value) in vect.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }
    
    //function parameters
    let point = (3, 5);
    print_coordinates(&point);




    //Refutability: Whether a Pattern Might Fail to Match
    let some_option_value: Option<i32> = None;
    if let Some(x) = some_option_value{    //If some_option_value was a None value, it would fail to match the pattern Some(x)
        println!("{}", x); //to fix the pattern on value None for Some(x), we use 'if let'
    }



    let y = 1;

    match y {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("anything"),
    }

    let a = Some(5);
    let b = 10;

    match a {
        Some(50) => println!("Got 50"),
        Some(b) => println!("Matched, b = {:?}", b),
        _ => println!("Default case, a = {:?}", a),
    }

    println!("at the end: a = {:?}, b = {:?}", a, b);


    //Multiple Patterns on match
    let c = 1;

    match c {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //Matching Ranges of Values with ..= {works for char values too}
    let d = 5;

    match d {
        1..=5 => println!("one through five"),  //range 1,2,3,4,5
        _ => println!("something else"),
    }

    
}

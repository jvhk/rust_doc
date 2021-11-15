
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
}

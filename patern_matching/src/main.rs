//function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

//Destructuring to Break Apart Values
struct Point {
    x: i32,
    y: i32,
}


//Destructuring Enums
enum Message { 
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(Color),
}


//Destructuring Nested Structs and Enums
enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}

//Ignoring entry balue with _
fn foo(_:i32, y:i32){
    println!("this code use only the y parameter: {}", y)
}

//Ignoring Remaining Parts of a Value with ..
struct New_point {
    x:i32,
    y:i32,
    z:i32,
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
    

    //Destructuring to Break Apart Values
    let p = Point {x: 0, y:7};
    
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);


    match p {
        Point { x, y:0 } => println!("on the x axis at {}",x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("on neither axis: {}, {} ",x,y),
    }

/*
    //Destructuring Enums
    let msg = Message::ChangeColor(0,160,255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move {x,y} => {
            println!("Move in the x direction {} and in the y direction {}",
                x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r,g,b) => println!(
          "Change the color to red {}, green {}, and blue {}", r,g,b
        ),
    }
*/
    //Destructuring Nested Structs and Enums
    let new_message = Message::ChangeColor(Color::Hsv(0 ,160, 255));

    match new_message {
        Message::ChangeColor(Color::Rgb(r,g,b)) => println!(
            "Change the color to red {}, green {}, and blue {}",r,g,b
        ),
        Message::ChangeColor(Color::Hsv(h,s,v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h,s,v
        ),
        _ => (),
    }

    //Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    

    //Ignoring parameter values with _  
    foo(3, 6);

    //Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2,3,4,5,6);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers ({}, {}, {})",first,third,fifth);
        }
    }

    //Ignoring an unused variable by starting this name with _
    let _ignored1 = 5;
    let _ignored2 = 3;
    
    //Ignoring Remaining Parts of a Value with ..
    let origin = New_point {x:0, y:0, z:0};

    match origin {
        New_point { x, .. } => println!("x is {}", x),
    }


    //Extra Conditionals with Match Guards
    let extra = Some(4);

    match extra {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

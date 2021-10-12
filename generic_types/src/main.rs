mod lib; //import file lib.rs
use lib::*;

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point_Two_Types<T, U>{
    x: T,
    y: U,
}

impl <T, U> Point_Two_Types<T,U>{
    fn mixup<V,W>(self, other: Point_Two_Types<V, W>) -> Point_Two_Types<T, W>{
        Point_Two_Types { 
            x: self.x, 
            y: other.y, 
        }
    }
}

//enums with generic types
enum Generic_Enum<T>{
    Some(T),
    None,
}

fn largest_i32(list: &[i32]) -> i32{   // 'list' type represents any concrete slice of i32
    let mut largest = list[0];

    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    largest //returns the largest number here
}

fn largest_char(list: &[char]) -> char{
    let mut largest = list[0];

    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    largest //returns the largest char here
}

// generic type of input to list


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    largest //returns the largest char here
}

fn main() {
    let list_of_i32 = vec![10,100,40,555,1,3];

    let result = largest_i32(&list_of_i32);
    println!("The largest list from 'i32_list' is {}", result);

    let list_of_char = vec!['y','x','a','b','m','z'];

    let result_char = largest_char(&list_of_char);
    println!("The largest list from 'char_list' is {}", result_char);


    //structs with generic types
    let integer_values  = Point{x: 2, y:3};
    println!("just_one_type_struct = {:?}", integer_values);

    let two_values  = Point_Two_Types{x: 2, y:3.3};
    println!("two_types_struct = {:?}", two_values);

    //mixed types in 'imp'
    let p1 = Point_Two_Types { x: 5.5, y: 1};
    let p2 = Point_Two_Types { x: 'c', y: "Hello mixed"};

    let value_mixed = p2.mixup(p1);

    println!("the value mixing p1 and p2 is {:#?}", value_mixed);

    /* 
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as probably already know, people"),
        reply: false,
        retweet: false,
    }; */

    let tweet = NewsArticle{
        headline: String::from("horse_ebooks"),
        location: String::from("of course, as probably already know, people"),
        author: String::from("ze"),
        content: String::from("some content") ,
    };

    println!("1 new tweet: {}", tweet.summarize());


    //Fixing largest function with Trait Bounds
    let number_list = vec![34, 50, 25, 100, 65];

    let result_largest = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result_largest = largest(&char_list);
    println!("The largest char is {}", result_largest);
}

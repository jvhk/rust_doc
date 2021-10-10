use std::{collections::HashMap, vec};

enum SpreedSheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    //creating a new vector
    let v: Vec<i32> = Vec::new();

    let vec = vec![3, 5, 3];

    //operations
    let mut vec2 = Vec::new();

    vec2.push(5);
    vec2.push(2);
    vec2.push(6);

    //reading the elements
    let third_element: &i32 = &vec[2];
    println!("The third element is {}" , third_element);

    match vec.get(2){
        Some(third_element) => println!("The third element is {}", third_element),
        None => println!("Theres no third element"),
    }

    for i in vec {
        println!("Element value {}", i);
    }

    let mut vec3 = vec![100, 32, 57];
        for i in &mut vec3 {
        *i += 50;
    }

    //enums to multiple types
    let row = vec![
        SpreedSheetCell::Int(3),
        SpreedSheetCell::Float(6.4),
        SpreedSheetCell::Text(String::from("Hola"))
    ];

    // hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    //creating a hash map from two lists
    let team = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores_map: HashMap<_,_> = 
        team.into_iter().zip(initial_scores.into_iter()).collect();
    
    // accessing values of a hash map
    let value_from_hash = scores.get("Blue");

    for (key, value) in &scores_map{
        println!("Key {} , Value {}", key, value);
    }

    //printing full hashmap
    println!("printing: scores hashmap: {:?}", scores_map);
    


}

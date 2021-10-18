/*
 
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32{ 
    println!("calculationg slowly.."); 
    thread::sleep(Duration::from_secs(2)); 
    intensity
} 

fn generate_workout(intensity: u32, random_number: u32) { 
    //let expensive_result= simulated_expensive_calculation(intensity);
    let expensive_closure = |num: u32| -> u32 { 
        println!("calculating slowly.."); 
        thread::sleep(Duration::from_secs(2)); 
        num
    }; 

    if intensity < 25 { 
        println!("Today, do {} pushups!",expensive_closure(intensity)); 
        println!("Next, do {} situps!", expensive_closure(intensity));  
    } else { 
        if random_number == 3 { 
            println!("Take a break today! Remember to stay hydrated!");
        } else { 
            println!("Today, run for {} minutes!", expensive_closure(intensity)); 
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_expensive_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_expensive_random_number);
}    
*/



// PROCESSING SERIES OF ITEMS WITH ITERATORS

fn main(){
    
    /* 
    let v1 = vec![1,2,3]; 
    let v1_iter = v1.iter();  //Creating an iterator

    for val in v1_iter{ 
        println!("Got: {}", val) 
    } */

    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    assert_eq!(v2, vec![2,3,4]);
}
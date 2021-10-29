use std::thread;
use std::time::Duration;


fn main() {

    // Creating a New Thread with spawn
    let handle = thread::spawn(|| { //Waiting for All Threads to Finish Using join Handles
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //handle.join().unwrap();

    

    //Using move Closures with Threads
    let v = vec![1,2,3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();
}

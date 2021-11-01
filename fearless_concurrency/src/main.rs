use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::rc::Rc;


fn main() {

    /*

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

*/

/*

    //Using Message Passing to Transfer Data Between Threads
    let (tx, rx) = mpsc::channel(); //sender / receiver

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    
    //Sending Multiple Values and Seeing the Receiver Waiting
    let (sen, rec) = mpsc::channel();

    let sen1 = sen.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            sen1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            sen.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rec {
        println!("Got: {}", received);
    }

*/

    //16.3Shared-State Concurrency
    //The API of Mutex<T>
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m); 


    //Sharing a Mutex<T> Between Multiple Threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter =  Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::io;

fn main() {
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();
    
    // Create a thread (simulating a fork)
    let receiver = Arc::new(Mutex::new(receiver));
    let receiver_clone = Arc::clone(&receiver);

    let child_thread = thread::spawn(move || {
        // Wait for the message from the parent
        let (mtext, number) = receiver_clone.lock().unwrap().recv().unwrap();
        println!("Child process read from msg: {}, {}", mtext, number);
    });

    // Parent thread
    let mut mtext = String::new();
    let mut number = String::new();

    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut mtext).unwrap();
    let mtext = mtext.trim(); // Trim whitespace/newlines

    println!("Please enter a number you want to send:");
    io::stdin().read_line(&mut number).unwrap();
    let number: i32 = number.trim().parse().unwrap();

    // Send the message to the child thread
    sender.send((mtext.to_string(), number)).unwrap();

    // Wait for the child thread to finish
    child_thread.join().unwrap();
}
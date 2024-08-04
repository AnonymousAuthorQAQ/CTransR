use std::io::{self, Write};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel for communication between parent and child
    let (tx, rx) = mpsc::channel();

    // Forking a new child process
    let child = thread::spawn(move || {
        // Wait for the message from the parent
        let (message, number) = rx.recv().unwrap();
        println!("Child process read from msg: {}, {}", message, number);
    });

    // Simulate the sleep in the parent process
    thread::sleep(Duration::from_secs(1));

    // Collect user inputs
    let mut input_text = String::new();
    let mut input_number = String::new();

    print!("Please enter a string you want to send: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed
    io::stdin().read_line(&mut input_text).expect("Failed to read line");

    print!("Please enter a number you want to send: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed
    io::stdin().read_line(&mut input_number).expect("Failed to read line");

    // Trim the input to remove newline characters
    let trimmed_text = input_text.trim().to_string();
    let trimmed_number: i32 = input_number.trim().parse().expect("Please enter a valid number");

    // Send the message to the child process
    tx.send((trimmed_text, trimmed_number)).unwrap();

    // Wait for the child thread to finish
    child.join().unwrap();
}
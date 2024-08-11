use std::io::{self, Write};
use std::process::{Command, exit};
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel for communication between processes
    let (sender, receiver) = mpsc::channel();

    // Spawn a child process
    let child = thread::spawn(move || {
        // Wait for the message from the parent
        let (text, number) = receiver.recv().expect("Failed to receive message");
        println!("Child process read from msg: {}, {}", text, number);
    });

    // Simulate the sleep (but it's not necessary here, since we use threads)
    thread::sleep(std::time::Duration::from_secs(1));

    // Get input from the user
    let mut input_text = String::new();
    print!("Please enter a string you want to send: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    // Trim the newline character
    let input_text = input_text.trim();

    let mut input_number = String::new();
    print!("Please enter a number you want to send: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read line");
    let input_number: i32 = input_number
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Send the message to the child process
    sender
        .send((input_text.to_string(), input_number))
        .expect("Failed to send message");

    // Wait for the child thread to finish
    child.join().expect("Child thread has panicked");

    // Exit the program
    exit(0);
}
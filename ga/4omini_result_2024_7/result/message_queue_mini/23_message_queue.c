use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let child = thread::spawn(move || {
        // Wait for the message from the parent thread
        let buff = rx.recv().unwrap();
        let (mtext, number) = buff;

        // Print the received message
        println!("Child process read from msg: {}, {}", mtext, number);
    });

    // Give some time for the child thread to start
    thread::sleep(std::time::Duration::from_millis(100));

    let mut mtext = String::new();
    let mut number = String::new();

    // Get input from user
    print!("Please enter a string you want to send: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mtext).expect("Failed to read line");
    mtext = mtext.trim().to_string(); // Remove the newline character

    print!("Please enter a number you want to send: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please enter a valid number");

    // Send the message and number to the child thread
    tx.send((mtext, number)).unwrap();

    // Wait for the child thread to finish
    child.join().unwrap();
}
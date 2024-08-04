use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use crossbeam_channel::{bounded, Receiver, Sender};

const MY_TYPE: usize = 9527;

fn main() {
    let (sender, receiver): (Sender<(String, i32)>, Receiver<(String, i32)>) = bounded(1);

    // Spawn a child process (thread in this case since Rust has no fork)
    let child_thread = thread::spawn(move || {
        // Wait for the parent to send a message
        let (message, number) = receiver.recv().unwrap();
        println!("Child process read from msg: {}, {}", message, number);
    });

    // Parent process
    println!("Please enter a string you want to send:");
    let mut message = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim().to_string(); // Trim newline characters

    println!("Please enter a number you want to send:");
    let mut number_string = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number_string).unwrap();
    let number: i32 = number_string.trim().parse().unwrap();

    // Send the data to the child thread
    sender.send((message, number)).unwrap();

    // Wait for the child thread to finish
    child_thread.join().unwrap();
}
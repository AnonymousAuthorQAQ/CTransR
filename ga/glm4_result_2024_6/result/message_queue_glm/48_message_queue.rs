use std::process;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::io::{self, Read};

fn main() {
    let (sender, receiver) = mpsc::channel();

    let input = thread::spawn(move || {
        let mut input = String::new();
        println!("Please enter a string you want to send: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.truncate(input.len() - 1); // Remove newline character
        println!("Please enter a number you want to send: ");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Please enter a valid number");

        (input, number)
    });

    thread::sleep(Duration::from_millis(1000));

    let result = input.join().expect("Failed to join thread");
    sender.send(result).unwrap();

    match receiver.try_recv() {
        Ok((text, number)) => println!("Child process read from msg: {}, {}", text, number),
        Err(TryRecvError::Empty) => println!("No message received"),
        Err(TryRecvError::Disconnected) => println!("Sender disconnected"),
    }
}
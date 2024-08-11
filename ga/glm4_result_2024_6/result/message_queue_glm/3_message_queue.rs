use std::io::{self, Read};
use std::thread;
use std::sync::mpsc;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let sender_clone = sender.clone();

    let handle = thread::spawn(move || {
        let mut input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut number: i32 = 0;
        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        number = input.trim().parse().expect("Please type a number!");
        sender_clone.send((input.trim().to_string(), number)).unwrap();
    });

    let mut input = String::new();
    println!("Child process is waiting for msg:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let (text, number) = receiver.recv().unwrap();
    println!("Child process read from msg: {}, {}", text, number);

    handle.join().unwrap();
}
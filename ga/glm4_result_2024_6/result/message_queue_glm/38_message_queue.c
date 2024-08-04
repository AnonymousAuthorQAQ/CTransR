use std::io::{self, Read};
use std::process::Command;
use std::ptr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let child = thread::spawn(move || {
        let (msg, number) = receiver.recv().unwrap();
        println!("Child process read from msg: {}, {}", msg, number);
    });

    thread::sleep(Duration::from_secs(1));

    let mut input = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    let mut number = 0;
    println!("Please enter a number you want to send:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    number = input.trim().parse().expect("Please type a number!");

    sender.send((input.to_string(), number)).unwrap();

    child.join().unwrap();
}
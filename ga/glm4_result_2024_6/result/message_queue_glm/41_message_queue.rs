use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx_child = tx.clone();
    let handle_child = thread::spawn(move || {
        let msg = rx.recv().unwrap();
        println!("Child process read from msg: {}", msg);
    });

    thread::sleep(Duration::from_secs(1));

    let mut input = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    println!("Please enter a number you want to send:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please type a number!");

    tx.send((input.to_string(), number)).unwrap();

    handle_child.join().unwrap();
}
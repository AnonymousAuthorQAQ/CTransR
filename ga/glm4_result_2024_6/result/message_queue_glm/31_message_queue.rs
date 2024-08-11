use std::io::{self, Read};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx_child = tx.clone();
    let handle = thread::spawn(move || {
        let mut input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();

        println!("Please enter a number you want to send:");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Please type a number!");

        tx_child.send((input, number)).expect("Failed to send message");
    });

    thread::sleep(Duration::from_secs(1));

    let (received_input, received_number) = rx.recv().expect("Failed to receive message");
    println!("Child process read from msg: {}, {}", received_input, received_number);

    handle.join().expect("Failed to join thread");
}
use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut input_text = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input_text).expect("Failed to read line");

        let mut input_number = String::new();
        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut input_number).expect("Failed to read line");
        let input_number: i32 = input_number.trim().parse().expect("Please type a number!");

        tx.send((input_text.trim().to_string(), input_number)).expect("Failed to send message");
    });

    thread::sleep(Duration::from_secs(1));

    if let Ok((received_text, received_number)) = rx.recv() {
        println!("Child process read from msg: {}, {}", received_text, received_number);
    }

    handle.join().expect("Failed to join thread");
}
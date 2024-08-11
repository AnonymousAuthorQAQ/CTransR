use std::io::{self, Read};
use std::os::raw::c_int;
use std::ptr;
use std::ffi::CString;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx_clone = tx.clone();
    let handle = thread::spawn(move || {
        let mut input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut number: i32 = 0;
        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        number = input.trim().parse().expect("Please type a number!");
        tx_clone.send((input.trim().to_string(), number)).unwrap();
    });

    thread::sleep(Duration::from_secs(1));

    if let Ok((text, number)) = rx.recv() {
        println!("Child process read from msg: {}, {}", text, number);
    }

    handle.join().unwrap();
}
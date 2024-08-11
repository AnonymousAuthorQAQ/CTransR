use std::io::{self, Read};
use std::os::raw::{c_long, c_int};
use std::ptr;
use std::process::{Command, Stdio};
use std::string::String;
use std::ffi::CString;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

const MY_TYPE: c_long = 9527;

fn main() {
    let (tx, rx) = channel();

    let tx_clone = tx.clone();
    let child_thread = thread::spawn(move || {
        let msg = rx.recv().unwrap();
        println!("Child process read from msg: {}", msg);
    });

    thread::sleep(Duration::from_secs(1));

    let msg = read_line("Please enter a string you want to send: ");
    let number = read_int("Please enter a number you want to send: ");
    tx_clone.send(format!("{} {}", msg, number)).unwrap();

    child_thread.join().unwrap();
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_int(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
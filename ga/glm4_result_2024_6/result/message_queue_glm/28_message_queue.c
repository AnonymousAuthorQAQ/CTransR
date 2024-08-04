use std::io::{self, Read};
use std::ptr;
use std::process::Command;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MY_TYPE: i32 = 9527;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let child = thread::spawn(move || {
        let msg = receiver.recv().unwrap();
        println!("Child process read from msg: {}", msg);
    });

    // Simulate the parent sending a message to the child
    thread::sleep(Duration::from_secs(1));
    let msg = "Hello, Rust!";
    sender.send(msg).unwrap();

    child.join().unwrap();
}

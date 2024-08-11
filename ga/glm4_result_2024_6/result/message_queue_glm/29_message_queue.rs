use std::io::{self, Read};
use std::os::unix::io::FromRawFd;
use std::ptr;
use std::ffi::CString;
use std::process::Command;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let msg = CString::new("Hello, world!").unwrap();
        let number = 42;
        tx1.send((msg, number)).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", str::from_utf8(received.0.as_bytes()).unwrap());
    println!("Number: {}", received.1);
}
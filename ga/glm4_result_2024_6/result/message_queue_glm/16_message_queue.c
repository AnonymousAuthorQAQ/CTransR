use std::io::{self, Read};
use std::process::exit;
use std::ptr::null;
use std::ptr::null_mut;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MY_TYPE: i32 = 9527;

fn main() {
    let (sender, receiver) = mpsc::channel::<(String, i32)>();

    let child_pid = unsafe { fork() };

    if child_pid > 0 {
        thread::sleep(Duration::from_secs(1));

        let mut input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: String = input.trim().to_string();

        println!("Please enter a number you want to send:");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Failed to parse number");

        sender.send((input, number)).expect("Failed to send message");

        exit(0);
    } else if child_pid == 0 {
        let (received_text, received_number) = receiver.recv().expect("Failed to receive message");
        println!("Child process read from msg: {}, {}", received_text, received_number);
    } else {
        eprintln!("Fork failed");
        exit(1);
    }
}

unsafe fn fork() -> i32 {
    extern "C" {
        fn fork() -> i32;
    }

    fork()
}
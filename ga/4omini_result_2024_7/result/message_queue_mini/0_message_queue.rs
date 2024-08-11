use std::io::{self, BufRead};
use std::process::{Command, exit};
use sysv_ipc::{MessageQueue, Key, FiniteQueue};

const MY_TYPE: i32 = 9527;

fn main() {
    // Create a message queue
    let key = Key::generate();
    let mq = MessageQueue::new(key, 0o600).expect("Failed to create message queue");

    // Fork the process
    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        let stdin = io::stdin();
        let mut input = String::new();

        // Read a string from stdin
        println!("Please enter a string you want to send:");
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        let input = input.trim(); // Trim newline

        // Read an integer
        println!("Please enter a number you want to send:");
        let mut number = String::new();
        stdin.lock().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Please enter a valid number");

        // Send message
        mq.send(&format!("{}:{}", input, number)).expect("Failed to send message");

        // Wait for child process to finish
        let _ = unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let msg = mq.receive::<String>().expect("Failed to receive message");
        let parts: Vec<&str> = msg.split(':').collect();
        let text = parts.get(0).unwrap_or(&"");
        let number: i32 = parts.get(1).unwrap_or(&"0").parse().expect("Failed to parse number");

        println!("Child process read from msg: {}, {}", text, number);
    } else {
        // Fork failed
        eprintln!("Failed to fork process");
        exit(1);
    }
}
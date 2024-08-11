use mqueue::{open, MessageQueue, QueueOptions};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::thread::sleep;
use std::time::Duration;

const MY_TYPE: u64 = 9527;

fn main() {
    let queue_name = "/example_mq";

    // Create the message queue
    let mq = MessageQueue::create(queue_name, QueueOptions::new().max_messages(10).max_message_size(256)).unwrap();

    // Fork a child process
    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        sleep(Duration::from_secs(1)); // Sleep to ensure child is ready

        let mut input_text = String::new();
        let mut input_number = String::new();

        // Get string input from the user
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_text).unwrap();

        // Get integer input from the user
        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_number).unwrap();
        let number: i32 = input_number.trim().parse().unwrap();

        // Prepare the message to send
        let message = format!("{}|{}", input_text.trim(), number);
        mq.send(message.as_bytes()).unwrap();

        unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) }; // Wait for the child process
    } else if pid == 0 {
        // Child process
        let mut buffer = [0u8; 256];
        let (size, _): (usize, _) = mq.receive(&mut buffer).unwrap();
        
        // Read the message received
        let received_data = String::from_utf8_lossy(&buffer[..size]);
        let parts: Vec<&str> = received_data.split('|').collect();
        
        if parts.len() == 2 {
            let msg_text = parts[0];
            let msg_number: i32 = parts[1].parse().unwrap();
            println!("Child process read from msg: {}, {}", msg_text, msg_number);
        }
    } else {
        eprintln!("Fork failed");
        exit(1);
    }
}
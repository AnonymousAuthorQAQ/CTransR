use mqueue::{Mqueue, MqueueOptions};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::str;
use std::thread;
use std::time::Duration;

const MY_TYPE: u64 = 9527; // type used in message

fn main() {
    let queue_name = "/example_queue";

    // Create a message queue
    let options = MqueueOptions::new().max_msg(10).msg_size(100);
    let mqueue = Mqueue::create(queue_name, options).expect("Failed to create message queue");

    let pid = unsafe { libc::fork() };

    if pid < 0 {
        eprintln!("Fork failed");
        exit(1);
    } else if pid > 0 {
        // Parent process
        thread::sleep(Duration::from_secs(1)); // wait for the child

        let mut input_string = String::new();
        let mut input_number = String::new();

        print!("Please enter a string you want to send: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input_string).expect("Failed to read input");
        input_string = input_string.trim().to_string();

        print!("Please enter a number you want to send: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input_number).expect("Failed to read input");
        let input_number: i32 = input_number.trim().parse().expect("Not a valid number");

        let msg = format!("{}|{}", input_string, input_number);
        mqueue.send(msg.as_bytes()).expect("Failed to send message");

        // Wait for the child process to finish
        let _ = unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
    } else {
        // Child process
        let mut buffer = vec![0; 100];
        let bytes_received = mqueue.recv(&mut buffer).expect("Failed to receive message");
        let received_msg = str::from_utf8(&buffer[..bytes_received]).expect("Failed to convert bytes to string");

        let split: Vec<&str> = received_msg.split('|').collect();
        if split.len() == 2 {
            let message = split[0];
            let number: i32 = split[1].parse().expect("Failed to parse number");
            println!("Child process read from msg: {}, {}", message, number);
        } else {
            println!("Received unexpected message format");
        }
    }
}
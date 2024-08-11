// To run this code, you need to declare the `msg` crate in your Cargo.toml
// [dependencies] 
// msg = "0.4"
use std::ffi::{CStr, CString};
use std::os::unix::process::CommandExt;
use std::process::{Command, exit};
use std::ptr::null_mut;
use std::{mem, ptr, str};
use std::io::{self, Write};
use msg::{MsgQueue, Message};
use std::time::Duration;
use std::thread;

const MY_TYPE: u32 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: u32,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    // Create the message queue
    let msg_queue = MsgQueue::create(0o666).expect("Failed to create message queue");

    // Fork the process using Command since Rust doesn't have `fork`
    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // This is the parent process
        thread::sleep(Duration::from_secs(1));

        // Prepare the message content
        let mut buffer = MsgBuf {
            mtype: MY_TYPE,
            mtext: Default::default(),
            number: 0,
        };

        // Get user input
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();

        // Read the string from input
        io::stdin().read_line(&mut buffer.mtext[..]).expect("Failed to read line");
        buffer.mtext = buffer.mtext[..buffer.mtext.len() - 1].to_owned(); // Remove newline

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();

        // Read the number from input
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input).expect("Failed to read line");
        buffer.number = number_input.trim().parse().expect("Please enter a valid number");

        // Send the message to the message queue
        let message = Message::new(&buffer, bufsize);
        msg_queue.send(message).expect("Failed to send message");

        // Wait for the child process to finish
        let _ = unsafe { libc::waitpid(pid, null_mut(), 0) };
    } else if pid == 0 {
        // This is the child process
        let mut received = MsgBuf {
            mtype: 0,
            mtext: Default::default(),
            number: 0,
        };

        // Receive the message from the message queue
        msg_queue.recv::<MsgBuf>().expect("Failed to receive message");
        println!(
            "Child process read from msg: {}, {}",
            str::from_utf8(&received.mtext).unwrap(),
            received.number
        );

        exit(0);
    } else {
        eprintln!("Fork failed");
        std::process::exit(1);
    }
}
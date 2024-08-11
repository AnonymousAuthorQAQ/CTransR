use nix::unistd::{fork, ForkType, wait, getpid};
use nix::sys::msg::{MsgQueue, Message, MsgFlags};
use std::io;
use std::ffi::CString;

const MY_TYPE: i64 = 9527;

fn main() {
    // Create a message queue
    let msg_queue = MsgQueue::new(None).expect("Failed to create message queue");

    // Create a fork
    match fork() {
        Ok(ForkType::Parent { child }) => {
            // In the parent process
            let mut input_text = String::new();
            let mut input_number = String::new();

            // Allow some time for child to set up (similar to sleep)
            std::thread::sleep(std::time::Duration::from_secs(1));
            
            // Input from user
            println!("Please enter a string you want to send:");
            io::stdin().read_line(&mut input_text).expect("Failed to read line");
            let c_string = CString::new(input_text.trim()).expect("CString::new failed");

            println!("Please enter a number you want to send:");
            io::stdin().read_line(&mut input_number).expect("Failed to read line");
            let number: i32 = input_number.trim().parse().expect("Please enter a valid number");

            // Create a message to send to the child
            let msg = Message {
                mtype: MY_TYPE,
                mtext: c_string.into_bytes(),
                number,
            };
            msg_queue.send(&msg, MsgFlags::empty()).expect("Failed to send message");
            
            // Wait for the child process to finish
            wait().expect("Failed to wait for child process");
        }
        Ok(ForkType::Child) => {
            // In the child process
            let mut msg = Message::default();
            msg_queue.receive(&mut msg, MsgFlags::empty()).expect("Failed to receive message");
            println!("Child process read from msg: {}, {}", String::from_utf8_lossy(&msg.mtext), msg.number);
        }
        Err(_) => {
            eprintln!("Fork failed");
        }
    }
}

// Define the `Message` struct to hold our message data
#[derive(Debug, Default)]
struct Message {
    mtype: i64,
    mtext: Vec<u8>,
    number: i32,
}

impl Message {
    pub fn new(mtype: i64, mtext: Vec<u8>, number: i32) -> Self {
        Message { mtype, mtext, number }
    }
}
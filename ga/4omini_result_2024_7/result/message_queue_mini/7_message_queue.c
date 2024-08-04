use std::io::{self, Write};
use std::process::Command;
use std::ptr;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::process::CommandExt;

#[derive(Debug)]
struct MsgBuf {
    mtype: i64,
    mtext: String,
    number: i32,
}

const MY_TYPE: i64 = 9527;

// Function to send the message
fn send_message(mq: &mq::MessageQueue, msg: MsgBuf) {
    let message_json = serde_json::to_string(&msg).expect("Failed to serialize message");
    mq.send(&message_json.as_bytes()).expect("Failed to send message");
}

// Function to receive the message
fn receive_message(mq: &mq::MessageQueue) -> MsgBuf {
    let mut buf = vec![0; 256]; // Allocate a buffer for the message
    let (len, _) = mq.receive(&mut buf).expect("Failed to receive message");
    let msg: MsgBuf = serde_json::from_slice(&buf[..len]).expect("Failed to deserialize message");
    msg
}

fn main() {
    // Create a message queue (using /tmp/mq as the queue name)
    let mq = mq::MessageQueue::create("/mq_example", mq::MessageQueueFlags::Create).unwrap();

    // Fork process
    unsafe {
        let pid = libc::fork();
        if pid > 0 {
            // Parent process
            println!("Please enter a string you want to send:");
            let mut mtext = String::new();
            io::stdin().read_line(&mut mtext).expect("Failed to read line");
            let mtext = mtext.trim(); // remove trailing newline

            println!("Please enter a number you want to send:");
            let mut number_input = String::new();
            io::stdin().read_line(&mut number_input).expect("Failed to read line");
            let number: i32 = number_input.trim().parse().expect("Invalid number");

            // Prepare message
            let message = MsgBuf {
                mtype: MY_TYPE,
                mtext: mtext.to_string(),
                number,
            };

            // Send message to child
            send_message(&mq, message);

            // Wait for the child process to exit
            libc::waitpid(pid, ptr::null_mut(), 0);

            // Clean up the message queue
            mq.unlink().expect("Failed to unlink message queue");
        } else if pid == 0 {
            // Child process
            println!("Child process is waiting for msg...");
            let received_msg = receive_message(&mq);
            println!(
                "Child process read from msg: {}, {}",
                received_msg.mtext, received_msg.number
            );

            // Exit child process
            std::process::exit(0);
        } else {
            eprintln!("Fork failed");
        }
    }
}
use std::io::{self, Write};
use std::process::{Command, Stdio};
use sysv_ipc::{msg_queue, MessageQueue, MsgQueueKey};
use std::os::unix::process::CommandExt;

const MY_TYPE: i32 = 9527;

fn main() -> io::Result<()> {
    // Create message queue
    let key = MsgQueueKey::generate();
    let mq = MessageQueue::new(key, sysv_ipc::Create::Create, 0o666)?;

    // Fork a child process
    let mut child = Command::new("rust_program")  // Replace with the actual name of compiled binary.
        .arg(format!("{}", key))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    // Parent Process
    let mut buffer = String::new();
    print!("Please enter a string you want to send: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    
    let mut number = String::new();
    print!("Please enter a number you want to send: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut number)?;
    
    let number: i32 = number.trim().parse().expect("Invalid number");

    // Send the message
    let msg = format!("{}|{}", buffer.trim(), number);
    mq.send(MY_TYPE, msg.as_bytes())?;
    
    // Wait for child process to finish
    let _ = child.wait()?;
    
    Ok(())
}

// Child process handling function
fn child_process(key: MsgQueueKey) {
    let mq = MessageQueue::open(key).unwrap();

    // Receive the message
    let (msg_type, msg) = mq.recv().unwrap();
    let message = String::from_utf8_lossy(&msg);
    let parts: Vec<&str> = message.split('|').collect();

    let received_string = parts[0];
    let received_number: i32 = parts[1].parse().expect("Invalid number");

    // Print the received message
    println!("Child process read from msg: {}, {}", received_string, received_number);
}
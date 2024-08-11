use std::io::{self, Read};
use std::process::Command;
use std::ptr::null;
use std::time::Duration;
use std::ffi::CString;
use ipc::{IpcResult, IpcPerm, MsgQueue, MsgBuf, MsgType};

fn main() {
    let msg_queue = match MsgQueue::new(IpcPerm::default(), 100) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Failed to create message queue: {}", e);
            return;
        }
    };

    let mut child = Command::new("sh")
        .arg("-c")
        .arg("rust_child")
        .spawn()
        .expect("Failed to spawn child process");

    // Wait for the child process to be ready
    std::thread::sleep(Duration::from_secs(1));

    let mut input = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    let mut number = 0;
    println!("Please enter a number you want to send:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    number = input.trim().parse().expect("Failed to parse number");

    let msg = MsgBuf {
        mtype: 9527,
        mtext: CString::new(input).unwrap(),
        number: number,
    };

    match msg_queue.send(&msg) {
        Ok(()) => println!("Message sent to child"),
        Err(e) => eprintln!("Failed to send message: {}", e),
    }

    let status = child.wait().expect("Failed to wait for child");
    println!("Child process exited with status: {}", status);
}

fn rust_child() {
    let msg_queue = match MsgQueue::open(9527) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Failed to open message queue: {}", e);
            return;
        }
    };

    let mut msg = MsgBuf {
        mtype: 9527,
        mtext: CString::new("".to_string()).unwrap(),
        number: 0,
    };

    match msg_queue.recv(&mut msg) {
        Ok(()) => println!("Child process read from msg: {}, {}", msg.mtext.to_string_lossy(), msg.number),
        Err(e) => eprintln!("Failed to receive message: {}", e),
    }
}
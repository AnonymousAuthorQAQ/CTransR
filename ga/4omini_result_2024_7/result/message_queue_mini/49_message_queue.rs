use nix::sys::msg::{
    MessageQueue,
    Msg,
};
use std::io;
use std::ptr;
use std::process::Command;
use std::mem::{size_of, zeroed};
use std::{process::exit, thread, time};

const MY_TYPE: i64 = 9527;

#[derive(Debug)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let (msg_queue, _) = MessageQueue::create(None).expect("Couldn't create message queue");
    
    let pid = unsafe { fork().expect("Fork failed") };
    
    if pid.is_parent() {
        // Parent process code
        thread::sleep(time::Duration::from_secs(1));
        
        let mut buff: MsgBuf = unsafe { zeroed() };
        buff.mtype = MY_TYPE;
        
        // Use reads for strings to avoid unsafe but require careful reads
        println!("Please enter a string you want to send:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed = input.trim();
        let bytes = trimmed.as_bytes();
        buff.mtext[..bytes.len()].copy_from_slice(bytes);

        println!("Please enter a number you want to send:");
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input).expect("Failed to read line");
        
        let number: i32 = number_input.trim().parse().expect("Please enter a valid number");
        buff.number = number;

        // Send the message
        msg_queue.send(&Msg::new(buff.mtype, &buff)).expect("Unable to send message");

        println!("Parent process sent: {} and {}", trimmed, number);

     } else {
        // Child process code
        let mut buff: MsgBuf = unsafe { zeroed() };
        
        // Receive message
        let msg: Msg<MsgBuf> = msg_queue
            .receive::<MsgBuf>(MY_TYPE)
            .expect("Failed to receive message");
        buff = msg.data();
        
        unsafe {
            println!("Child process read from msg: {}, {}", std::str::from_utf8_unchecked(&buff.mtext).trim_end_matches(char::from(0)), buff.number);
        }
    }
}
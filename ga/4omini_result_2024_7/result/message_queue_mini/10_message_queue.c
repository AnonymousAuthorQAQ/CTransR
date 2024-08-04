use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::process::{Command, exit};
use nix::msg::MsgBuf;
use nix::msg::{MessageQueue, Message};
use nix::unistd::{fork, ForkFlag, execve, sleep, wait, ForkResult};
use std::ptr;

const MY_TYPE: i32 = 9527;

#[derive(Debug)]
struct Msgbuf {
    mtype: i32,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let msgid = MessageQueue::new().expect("msgget failed");

    let pid = unsafe { fork().expect("fork failed") };

    match pid {
        ForkResult::Parent { child } => {
            // Parent process
            sleep(1);

            let mut buff = Msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Get user input for mtext
            print!("Please enter a string you want to send: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut String::from_utf8_lossy(&mut buff.mtext).to_string()).expect("Failed to read line");

            // Get user input for number
            print!("Please enter a number you want to send: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut number_input = String::new();
            io::stdin().read_line(&mut number_input).expect("Failed to read line");
            buff.number = number_input.trim().parse::<i32>().expect("Invalid number");

            msgid.send(Message {
                mtype: buff.mtype,
                mtext: buff.mtext.to_vec(),
                ..Default::default()
            }).expect("msgsnd failed");

            // Wait for child process to terminate
            wait().expect("wait failed");
        },
        ForkResult::Child => {
            // Child process
            let mut buff = Msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            let message: MsgBuf<[u8; 100]> = msgid.recv(MY_TYPE).expect("msgrcv failed");
            buff.mtext.copy_from_slice(&message.mtext);
            buff.number = message.number;

            // Print the received message
            println!("Child process read from msg: {}, {}", String::from_utf8_lossy(&buff.mtext), buff.number);
            exit(0);
        }
    }
}
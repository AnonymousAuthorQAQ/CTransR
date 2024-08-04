extern crate nix;
use nix::sys::msg;
use nix::sys::msg::{msgsnd, msgrcv, MsgType, MsgFlags};
use nix::sys::types::key_t;
use std::ffi::CString;
use std::ptr;
use nix::unistd::{fork, getpid, pause};
use libc::{sleep, WEXITED, waitpid};

const MY_TYPE: i64 = 9527;

fn main() {
    let key: key_t = 0x1234; // Unique identifier for the message queue
    let msgid = msg::msgget(key, msg::MsgFlags::IPC_CREAT | msg::MsgFlags::S_IWUSR | msg::MsgFlags:        :S_IRUSR).unwrap();

    match fork() {
        Ok(fork_result) => {
            if fork_result.is_parent() {
                sleep(1);

                let msg_data = "Hello Rust".to_string();
                let buf = msg::Msghdr::new(
                    CString::new(&msg_data).unwrap(),
                    MY_TYPE,
                    0
                );
                
                msgsnd(msgid, &buf, MsgType::ANY, MsgFlags::empty()).unwrap();

                let mut status: i32 = 0;
                waitpid(fork_result.as_raw(), &mut status, WEXITED).unwrap();
            } else {
                let msg_received = "                            ".to_string(); // buffer to hold received message
                
                // Receive the message from the queue
                let mut buf = msg::Msghdr::new(
                    CString::new(&msg_received).unwrap(),
                    MY_TYPE,
                    0
                );
                msgrcv(msgid, &mut buf, MsgType::of(0), MsgFlags::empty(), 0).unwrap();
                let received_msg = std::str::from_utf8(CString::new(&msg_received).unwrap().as_bytes()).unwrap();
                println!("Child process read from msg: {}", received_msg.trim());
            }
        }
        Err(_) => {
            eprintln!("Fork Error");
        }
    }

    msg::msgctl(msgid, msg::IpcCommand::IpcRm, None).unwrap(); // Remove the message queue after use
}
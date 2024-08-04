use std::io::{self, Write};
use std::os::unix::io::{RawFd, AsRawFd};
use std::process::{Command, exit};
use std::ptr::{null_mut};
use std::ffi::CString;

extern crate nix;
use nix::errno::Errno;
use nix::sys::msg::{MsgQueue, MqAck, MqSendFlags, MqRecvFlags};

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [c_char; 100],
    number: i32,
}

fn main() {
    let msg_queue = MsgQueue::new();
    if msg_queue.is_err() {
        eprintln!("msgget failed");
        return;
    }

    let msg_queue = msg_queue.unwrap();

    // Creating a child process
    let pid = unsafe { libc::fork() };

    match pid {
        0 => { // Child process
            let mut buff: MsgBuf = unsafe { std::mem::zeroed() };
            
            match msg_queue.recv(
                &mut buff,
                std::mem::size_of::<MsgBuf>() as usize,
                MqRecvFlags::empty(),
            ) {
                Ok(_) => {
                    let message = unsafe { CStr::from_ptr(buff.mtext.as_ptr()) };
                    println!("Child process read from msg: {}, {}", message.to_str().unwrap(), buff.number);
                },
                Err(e) => eprintln!("msgrcv failed: {:?}", e),
            }
        },
        -1 => {
            eprintln!("fork failed");
            return;
        },
        _ => { // Parent process
            let mut buff = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Read string input
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buff.mtext).unwrap();

            // Read integer input
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buf).unwrap();
            buff.number = buf.trim().parse().unwrap();

            match msg_queue.send(&buff, MqSendFlags::empty()) {
                Ok(_) => { /* message sent successfully */ }
                Err(e) => eprintln!("msgsnd failed: {:?}", e),
            }

            let _ = unsafe { libc::waitpid(pid, null_mut(), 0) }; // Wait for child process
        }
    }
}
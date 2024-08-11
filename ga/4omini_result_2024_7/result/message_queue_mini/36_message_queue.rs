use nix::libc;
use nix::msg::{MsqId, MsgBuf, msgsnd, msgrcv, MsgQueue};
use nix::sys::ipc::{MsgQueue as NixMsgQueue, key_t, IPC_PRIVATE};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::ptr;
use std::os::unix::process::CommandExt;

const MY_TYPE: i64 = 9527;

#[derive(Debug)]
struct MsgBufWrapper {
    mtype: i64,
    mtext: [i8; 100],
    number: i32,
}

fn main() {
    let key = IPC_PRIVATE;
    let msg_queue = NixMsgQueue::new(key, 0).expect("Failed to create message queue");
    
    let pid = unsafe { libc::fork() };
    
    match pid {
        0 => {
            // Child process
            let mut buf: MsgBufWrapper = unsafe { std::mem::zeroed() };

            // Receive message
            let result = msgrcv(msg_queue, &mut buf as *mut _ as *mut libc::c_void, 
                                std::mem::size_of::<MsgBufWrapper>() - std::mem::size_of::<i64>(), 
                                MY_TYPE, 0);
            match result {
                Ok(_) => {
                    let text = unsafe { std::ffi::CStr::from_ptr(buf.mtext.as_ptr()) }
                        .to_string_lossy()
                        .into_owned();
                    println!("Child process read from msg: {}, {}", text, buf.number);
                },
                Err(err) => {
                    eprintln!("msgrcv failed: {}", err);
                }
            }
            exit(0);
        },
        -1 => {
            eprintln!("fork failed");
            exit(1);
        },
        _ => {
            // Parent process
            let mut buffer = String::new();
            let mut number = String::new();
            
            // Simulating sleep
            std::thread::sleep(std::time::Duration::from_secs(1));
            
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer).unwrap();
            
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut number).unwrap();
            
            let number: i32 = number.trim().parse().expect("Please enter a valid number");
            
            let mut msg_buf = MsgBufWrapper {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number,
            };
            let bytes = buffer.trim().as_bytes();
            for (i, &byte) in bytes.iter().enumerate() {
                msg_buf.mtext[i] = byte as i8;
            }
            // Send message
            msgsnd(msg_queue, &msg_buf as *const _ as *const libc::c_void, 
                    std::mem::size_of::<MsgBufWrapper>() - std::mem::size_of::<i64>(), 0)
                .expect("msgsnd failed");
            
            // Wait for child process to finish
            unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
        }
    }
}
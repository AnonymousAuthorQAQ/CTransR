extern crate libc;

use std::io::{self, Write};
use std::ptr;
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;

const MY_TYPE: i32 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i32,
    mtext: [i8; 100], // Message text
    number: i32,
}

fn main() {
    unsafe {
        let msgid = libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL);
        if msgid == -1 {
            eprintln!("msgget failed");
            exit(1);
        }

        // Fork a child process
        let pid = libc::fork();
        if pid > 0 {
            // Parent process
            libc::sleep(1);

            let mut buff = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Input string
            println!("Please enter a string you want to send:");
            io::stdin().read_line(&mut buff.mtext[..])
                .expect("Failed to read line");

            // Input number
            println!("Please enter a number you want to send:");
            let mut number_input = String::new();
            io::stdin().read_line(&mut number_input)
                .expect("Failed to read line");
            buff.number = number_input.trim().parse().expect("Invalid number");

            // Send message
            let result = libc::msgsnd(msgid, &mut buff as *mut _ as *const _, 
                std::mem::size_of::<MsgBuf>() as isize - std::mem::size_of::<i32>() as isize, 0);
            if result == -1 {
                eprintln!("msgsnd failed");
                exit(1);
            }

            // Wait for child to finish
            libc::waitpid(pid, ptr::null_mut(), 0);
        } else if pid == 0 {
            // Child process
            let mut buff = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            // Receive message
            let result = libc::msgrcv(msgid, &mut buff as *mut _ as *mut _, 
                std::mem::size_of::<MsgBuf>() as isize - std::mem::size_of::<i32>() as isize, 
                MY_TYPE, 0);
            if result == -1 {
                eprintln!("msgrcv failed");
                exit(1);
            }

            // Print received message
            let message = CString::from_raw(buff.mtext.as_mut_ptr());
            println!("Child process read from msg: {}, {}", message.to_string_lossy(), buff.number);
        } else {
            eprintln!("fork failed");
            exit(1);
        }

        // Cleanup message queue (optional based on your use case)
        libc::msgctl(msgid, libc::IPC_RMID, ptr::null_mut());
    }
}
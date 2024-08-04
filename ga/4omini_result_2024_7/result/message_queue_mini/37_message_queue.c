extern crate libc;

use std::ffi::{CString, CStr};
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::os::unix::process::CommandExt;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use std::ptr::null_mut;

const MY_TYPE: i32 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i32,
    mtext: [i8; 100],
    number: i32,
}

fn main() -> libc::c_int {
    unsafe {
        // Create the message queue
        let msgid = libc::msgget(libc::IPC_PRIVATE, libc::IPC_CREAT | 0o666);
        if msgid == -1 {
            eprintln!("msgget error");
            return -1;
        }

        // Fork the process
        let pid = libc::fork();
        if pid > 0 {
            // Parent process
            libc::sleep(1); // Sleep to ensure child process is ready

            // Get user input
            let mut buff = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Read string input
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let c_string = CString::new(input.trim()).expect("CString::new failed");
            let bytes = c_string.as_bytes_with_nul();
            buff.mtext[..bytes.len()].copy_from_slice(bytes);

            // Read number input
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            buff.number = input.trim().parse().unwrap_or(0);

            // Send message to the queue
            let result = libc::msgsnd(msgid, &mut buff as *mut MsgBuf as *const _, 
                                       mem::size_of::<MsgBuf>() as usize - mem::size_of::<i32>() as usize,
                                       0);
            if result == -1 {
                eprintln!("msgsnd error");
                return -1;
            }

            // Wait for the child process
            libc::waitpid(pid, null_mut(), 0);
        } else if pid == 0 {
            // Child process
            let mut buff = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            // Receive the message
            let result = libc::msgrcv(msgid, &mut buff as *mut MsgBuf as *mut _, 
                                       mem::size_of::<MsgBuf>() as usize - mem::size_of::<i32>() as usize,
                                       MY_TYPE, 0);
            if result == -1 {
                eprintln!("msgrcv error");
                return -1;
            }

            // Convert and print received message
            let received_string = CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy();
            println!("Child process read from msg: {}, {}", received_string, buff.number);
        } else {
            // Fork error
            eprintln!("fork error");
            return -1;
        }

        // Clean up the message queue
        libc::msgctl(msgid, libc::IPC_RMID, ptr::null_mut());

        0
    }
}
use std::io::{self, Read};
use std::process::exit;
use std::ptr::null;
use std::ffi::CString;
use std::os::raw::{c_int, c_long};
use std::ptr;

use libc::{fork, waitpid, msgrcv, msgsnd, msgget, IPC_PRIVATE, IPC_EXCL, IPC_CREAT, IPC_NOWAIT, msgctl, IPC_RMID, fork, waitpid};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct Msgbuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}

fn main() {
    let mut buffer = Msgbuf {
        mtype: MY_TYPE,
        mtext: [0; 100],
        number: 0,
    };

    let msgid = unsafe {
        msgget(
            IPC_PRIVATE,
            IPC_CREAT | IPC_EXCL | 0666,
        )
    };

    if msgid == -1 {
        eprintln!("msgget failed");
        exit(1);
    }

    match unsafe { fork() } {
        -1 => {
            eprintln!("fork failed");
            exit(1);
        }
        0 => {
            // child
            loop {
                let result = unsafe {
                    msgrcv(msgid, &mut buffer as *mut _ as *mut _, 104, MY_TYPE, 0)
                };

                if result == -1 {
                    eprintln!("msgrcv failed");
                    break;
                }

                println!("Child received: {}", buffer.mtext.to_str().unwrap());
                println!("Child received number: {}", buffer.number);
            }
        }
        pid => {
            // parent
            println!("Please enter a string to send:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = CString::new(input.trim()).expect("Failed to create CString");

            println!("Please enter a number to send:");
            let mut number = String::new();
            io::stdin().read_line(&mut number).expect("Failed to read line");
            let number: c_int = number.trim().parse().expect("Failed to parse number");

            buffer.mtext[..input.as_bytes().len()].copy_from_slice(input.as_bytes());
            buffer.number = number;

            unsafe {
                msgsnd(msgid, &buffer as *const _ as *const _, 104, 0);
            }

            unsafe {
                waitpid(pid, null_mut(), 0);
            }

            unsafe {
                msgctl(msgid, IPC_RMID, null_mut());
            }
        }
    }
}
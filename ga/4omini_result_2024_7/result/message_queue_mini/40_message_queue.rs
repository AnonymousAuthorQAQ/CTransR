extern crate libc;

use std::ffi::CString;
use std::io::{self, Write};
use std::ptr;
use std::os::unix::io::RawFd;
use std::process::{Command, exit};

const MY_TYPE: i32 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [i8; 100],
    number: i32,
}

fn main() {
    unsafe {
        let msgid = libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL);
        if msgid == -1 {
            eprintln!("msgget failed");
            exit(-1);
        }

        let pid = libc::fork();
        if pid > 0 {
            // Parent process
            libc::sleep(1);

            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE as i64,
                mtext: [0; 100],
                number: 0,
            };

            // Get input from user
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut String::from_utf8_lossy(&mut buff.mtext[..]).to_string()).unwrap();
            
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            let mut input_number = String::new();
            io::stdin().read_line(&mut input_number).unwrap();
            buff.number = input_number.trim().parse().unwrap();

            libc::msgsnd(msgid, &mut buff as *mut _ as *const _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0);

            libc::waitpid(pid, ptr::null_mut(), 0);
        } else if pid == 0 {
            // Child process
            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE as i64,
                mtext: [0; 100],
                number: 0,
            };

            libc::msgrcv(msgid, &mut buff as *mut _ as *mut _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), MY_TYPE as i64, 0);
            println!("Child process read from msg: {}, {}", CString::from_raw(buff.mtext.as_mut_ptr()).to_string_lossy(), buff.number);
        } else {
            eprintln!("fork failed");
            exit(-1);
        }
    }
}
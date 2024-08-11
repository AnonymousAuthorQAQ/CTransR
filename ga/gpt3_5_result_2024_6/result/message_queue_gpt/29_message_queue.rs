use std::ffi::{CString, CStr};
use std::os::raw::{c_long, c_int};
use std::mem;
use libc::{key_t, msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL};
use std::ptr;
use std::process::Command;
use std::io::{self, Write};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct msgbuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: c_int,
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error creating message queue");
        return;
    }

    let mut pid = unsafe { libc::fork() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff = msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };
        
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.pop(); // remove newline

        let c_input = CString::new(input).unwrap();
        let input_bytes = c_input.as_bytes();
        buff.mtext[..input_bytes.len()].copy_from_slice(input_bytes);

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input).unwrap();
        let number: c_int = num_input.trim().parse().unwrap();
        buff.number = number;

        unsafe {
            msgsnd(msgid, &buff as *const msgbuf as *const _, mem::size_of::<msgbuf>() - mem::size_of::<c_long>(), 0);
        }

        Command::new("wait").arg(pid.to_string()).output().expect("failed to wait for child process");
    } else if pid == 0 {
        let mut buff = msgbuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        println!("Child process is waiting for message...");
        unsafe {
            msgrcv(msgid, &buff as *mut msgbuf as *mut _, mem::size_of::<msgbuf>() - mem::size_of::<c_long>(), MY_TYPE, 0);
        }
        
        let c_str = CStr::from_ptr(buff.mtext.as_ptr());
        let str_val = c_str.to_str().expect("Invalid UTF-8 string");
        println!("Child process read from message: {}, {}", str_val, buff.number);
    } else {
        eprintln!("Fork failed");
    }
}
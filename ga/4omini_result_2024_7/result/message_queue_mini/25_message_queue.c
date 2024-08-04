extern crate libc;

use libc::{c_void, msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_CREAT, EINVAL};
use std::ffi::{CString, CStr};
use std::io::{self, Write};
use std::mem;
use std::process;
use std::ptr;

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [i8; 100],
    number: i32,
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_CREAT) };

    if msgid == -1 {
        eprintln!("msgget failed");
        process::exit(1);
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        std::thread::sleep(std::time::Duration::from_secs(1)); // sleep(1)

        let mut buff = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100], // Initialize the mtext array
            number: 0,
        };

        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap(); // flush stdout

        // Read input for mtext
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let c_str = CString::new(input.trim()).expect("CString::new failed");
        buff.mtext[..c_str.as_bytes().len()].copy_from_slice(c_str.as_bytes());

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input).unwrap();
        buff.number = number_input.trim().parse().unwrap();

        // Sends the message
        let result = unsafe {
            msgsnd(msgid, &buff as *const _ as *const c_void, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), 0)
        };

        if result == -1 {
            eprintln!("msgsnd failed");
        }

        unsafe { libc::waitpid(pid, ptr::null_mut(), 0) }; // Wait for the child process
    } else if pid == 0 {
        // Child process
        let mut buff = MsgBuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        // Receive the message
        let result = unsafe {
            msgrcv(msgid, &mut buff as *mut _ as *mut c_void, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), MY_TYPE, 0)
        };

        if result == -1 {
            eprintln!("msgrcv failed");
        } else {
            let msg = unsafe { CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy().into_owned() };
            println!("Child process read from msg: {}, {}", msg, buff.number);
        }
    } else {
        eprintln!("fork failed");
    }
}
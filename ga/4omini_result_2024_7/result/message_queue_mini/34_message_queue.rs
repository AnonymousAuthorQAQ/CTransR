extern crate libc;

use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::ptr;
use std::mem;
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [c_char; 100],
    number: i32,
}

fn main() {
    let mut msgid: i32 = unsafe { libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL) };

    if msgid == -1 {
        eprintln!("msgget failed");
        exit(1);
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        let mut buff: MsgBuf = unsafe { mem::zeroed() };

        // Allow a brief sleep to ensure the child is ready to receive
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Get user input
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();

        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();

        // Prepare data structure to send
        buff.mtype = MY_TYPE;
        
        // Copy the string into mtext, up to 100 bytes
        let c_str = CString::new(input_string.trim()).unwrap();
        let bytes = c_str.as_bytes_with_nul();
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), buff.mtext.as_mut_ptr(), bytes.len().min(mem::size_of_val(&buff.mtext)));
        }

        // For safer number input in C
        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();

        let mut input_number = String::new();
        io::stdin().read_line(&mut input_number).unwrap();
        buff.number = input_number.trim().parse().unwrap_or(0);

        unsafe {
            libc::msgsnd(msgid, &buff as *const _ as *const _, mem::size_of::<MsgBuf>() as i32 - mem::size_of::<i64>() as i32, 0);
        }

        // Wait for the child process to finish
        unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        let mut buff: MsgBuf = unsafe { mem::zeroed() };

        unsafe {
            if libc::msgrcv(msgid, &mut buff as *mut _ as *mut _, mem::size_of::<MsgBuf>() as i32 - mem::size_of::<i64>() as i32, MY_TYPE, 0) == -1 {
                eprintln!("msgrcv failed");
                exit(1);
            }
        }

        let message = unsafe { CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy() };
        println!("Child process read from msg: {}, {}", message, buff.number);
    } else {
        eprintln!("fork failed");
        exit(1);
    }
}
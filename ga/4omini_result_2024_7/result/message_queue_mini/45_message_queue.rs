use libc::{c_int, c_char, c_long, size_t, msgget, msgsnd, msgrcv, waitpid, fork, sleep};
use std::ffi::CString;
use std::io::{self, Write};
use std::mem::zeroed;
use std::os::unix::prelude::*;
use std::ptr;
use std::process;
use std::os::unix::io::{AsRawFd, FromRawFd};

// Define message structure
#[repr(C)]
struct Msgbuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: i32,
}

const IPC_PRIVATE: c_int = -1;
const MY_TYPE: c_long = 9527;

fn main() {
    unsafe {
        // Create a message queue
        let msgid = msgget(IPC_PRIVATE, 0o666 | libc::IPC_EXCL);
        if msgid == -1 {
            eprintln!("msgget failed");
            process::exit(1);
        }

        // Fork the process
        let pid = fork();
        if pid < 0 {
            eprintln!("Fork failed");
            process::exit(1);
        } else if pid > 0 {
            // Parent process
            sleep(1);

            let mut buff: Msgbuf = zeroed();
            buff.mtype = MY_TYPE;

            // Read string and number from user
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            // Use the line of code below once the proper limits are set.
            let mut input_string = String::new();
            io::stdin().read_line(&mut input_string).expect("Failed to read line");
            let c_string = CString::new(input_string.trim()).expect("CString::new failed");
            input_string.truncate(100); // Ensure not to exceed buffer size - it's not Rust's job to enforce C semantics.
            input_string.as_ptr().copy_to(buff.mtext.as_mut_ptr(), input_string.len());

            print!("Please enter a number you want to send: ");
            io::stdout().flush().ok();
            let mut input_number = String::new();
            io::stdin().read_line(&mut input_number).expect("Failed to read line");
            buff.number = input_number.trim().parse().expect("Failed to parse number");

            // Send the message
            if msgsnd(msgid, &mut buff as *mut _ as *const _, std::mem::size_of::<Msgbuf>() - std::mem::size_of::<c_long>(), 0) == -1 {
                eprintln!("msgsnd failed");
                process::exit(1);
            }

            // Wait for child
            waitpid(pid, ptr::null_mut(), 0);
        } else {
            // Child process
            let mut buff: Msgbuf = zeroed();

            // Receive the message
            if msgrcv(msgid, &mut buff as *mut _ as *mut _, std::mem::size_of::<Msgbuf>() - std::mem::size_of::<c_long>(), MY_TYPE, 0) == -1 {
                eprintln!("msgrcv failed");
                process::exit(1);
            }

            // Print the received message
            println!("Child process read from msg: {} {}", 
                     CString::from_raw(buff.mtext.as_mut_ptr()).to_string_lossy(), 
                     buff.number);
        }
    }
}
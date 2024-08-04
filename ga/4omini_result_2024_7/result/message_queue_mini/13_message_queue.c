use libc::{c_long, c_int, c_void, IPC_PRIVATE, msgget, msgsnd, msgrcv, waitpid, fork, perror, WNOHANG};
use std::ffi::{CString, CStr};
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::{RawFd};
use std::process::{Command, exit};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long, // message type
    mtext: [c_char; 100], // message text
    number: c_int, // additional integer
}

fn main() {
    let msgid: RawFd;

    unsafe {
        msgid = msgget(IPC_PRIVATE, 0o666 | libc::IPC_EXCL);
        if msgid == -1 {
            perror(CString::new("msgget").unwrap().as_ptr());
            exit(-1);
        }

        let pid = fork();

        if pid > 0 {
            // Parent process
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Read string from input
            print!("Please enter a string you want to send:\n");
            io::stdout().flush().unwrap(); // Ensure the prompt is printed
            let mut input_string = String::new();
            io::stdin().read_line(&mut input_string).expect("Failed to read line");
            let input_bytes = input_string.as_bytes();
            let len = std::cmp::min(input_bytes.len(), 99); // 99 because of terminator
            buff.mtext[..len].copy_from_slice(&input_bytes[..len]);
            buff.mtext[len] = 0; // Null terminate

            // Read integer from input
            print!("Please enter a number you want to send:\n");
            io::stdout().flush().unwrap(); // Ensure the prompt is printed
            io::stdin().read_line(&mut input_string).expect("Failed to read line");
            buff.number = input_string.trim().parse().unwrap_or(0); // Default to 0 in case of error

            msgsnd(msgid, &buff as *const _ as *const c_void, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), 0);

            // Wait for child
            waitpid(pid, std::ptr::null_mut(), WNOHANG);
        } else if pid == 0 {
            // Child process
            let mut buff: MsgBuf = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            msgrcv(msgid, &mut buff as *mut _ as *mut c_void, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), MY_TYPE, 0);
            let message_str = unsafe { CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy() };

            println!("Child process read from msg: {}, {}", message_str, buff.number);
            // Clean up the message queue (optional)
            // let _ = libc::msgctl(msgid, libc::IPC_RMID, std::ptr::null_mut());
        } else {
            // Fork failed
            perror(CString::new("fork").unwrap().as_ptr());
        }
    }
}
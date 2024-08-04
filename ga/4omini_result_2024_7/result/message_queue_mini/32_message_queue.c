use std::ffi::CString;
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::process::{Command, exit};
use std::ptr;
use std::ptr::null_mut;
use std::os::unix::process::CommandExt;
use std::os::raw::c_int;

#[repr(C)]
struct Msgbuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}

const MY_TYPE: c_long = 9527;

extern "C" {
    fn msgget(key: c_int, msgflg: c_int) -> c_int;
    fn msgsnd(msqid: c_int, msgp: *const Msgbuf, msgsz: c_int, msgflg: c_int) -> c_int;
    fn msgrcv(msqid: c_int, msgp: *mut Msgbuf, msgsz: c_int, msgtype: c_long, msgflg: c_int) -> c_int;
}

fn main() {
    unsafe {
        // Create a message queue with IPC_PRIVATE key
        let msgid = msgget(IPC_PRIVATE, 0o666 | IPC_EXCL);
        if msgid == -1 {
            eprintln!("msgget failed");
            exit(1);
        }

        match libc::fork() {
            0 => { // Child process
                // Prepare msg buffer to receive
                let mut buff: Msgbuf = Msgbuf {
                    mtype: 0,
                    mtext: [0; 100],
                    number: 0,
                };

                // Receive the message
                msgrcv(msgid, &mut buff as *mut _ , std::mem::size_of::<Msgbuf>() as c_int - std::mem::size_of::<c_long>() as c_int , MY_TYPE, 0);
                let msg_str = std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy();
                println!("Child process read from msg: {}, {}", msg_str, buff.number);
            }
            -1 => {
                eprintln!("fork failed");
                exit(1);
            }
            _ => { // Parent process
                // Sleep for a moment to allow the child process to be ready
                std::thread::sleep(std::time::Duration::from_secs(1));

                // Get user input for message
                let mut input_string = String::new();
                let mut number_input = String::new();

                // Read the input string
                println!("Please enter a string you want to send:");
                io::stdin().read_line(&mut input_string).unwrap();
                // Clear any trailing newline
                let input_bytes = input_string.as_bytes();
                for i in input_bytes.len()..100 {
                    buff.mtext[i] = 0; // Null terminate the char array if not filled
                }

                // Read the input number
                println!("Please enter a number you want to send:");
                io::stdin().read_line(&mut number_input).unwrap();
                let trimmed_number_input = number_input.trim();
                buff.number = trimmed_number_input.parse().unwrap_or(0); // Fall back to 0 in case of an error
                buff.mtype = MY_TYPE;

                // Send the message
                msgsnd(msgid, &buff as *const _ , std::mem::size_of::<Msgbuf>() as c_int - std::mem::size_of::<c_long>() as c_int, 0);
            }
        }
    }
}
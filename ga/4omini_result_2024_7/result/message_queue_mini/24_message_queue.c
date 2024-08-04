use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, ExitStatus};
use std::ptr;
use std::mem::{self, MaybeUninit};
use libc::{c_long, msgrcv, msgsnd, IPC_PRIVATE, msgget, Msg, waitpid, sleep, EINVAL};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [u8; 100],
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | libc::IPC_CREAT | libc::IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Failed to create message queue");
        return;
    }

    // Fork the process
    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        unsafe {
            sleep(1); // Wait for child process to be ready
        }

        let mut buff = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
        };

        // Read string input
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed
        
        io::stdin().read_line(&mut buff.mtext.iter_mut().take(100).collect::<Vec<u8>>()).unwrap();

        // Read integer input
        let mut number = String::new();
        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut number).unwrap();
        let num: i32 = number.trim().parse().expect("Please enter a valid number");

        // Fill the buffer
        let slice = &mut buff.mtext;
        let size_of_data = mem::size_of::<MsgBuf>() - mem::size_of::<c_long>();
        unsafe {
            (*(slice.as_mut_ptr() as *mut MsgBuf)).mtype = MY_TYPE;
            (*ptr::slice_from_raw_parts(slice.as_ptr(), num as usize)).copy_from_nonoverlapping(buff.mtext.as_ptr(), data.len());
            
            if msgsnd(msgid, &buff as *const _ as _, size_of_data, 0) == -1 {
                eprintln!("Error sending message");
            };
        }

        // Wait for the child process to finish
        let _ = unsafe { waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let mut buff: MsgBuf = unsafe { MaybeUninit::zeroed().assume_init() };
        let size_of_data = mem::size_of::<MsgBuf>() - mem::size_of::<c_long>();
        
        unsafe {
            if msgrcv(msgid, &mut buff as *mut MsgBuf as *mut _, size_of_data, MY_TYPE, 0) == -1 {
                eprintln!("Error receiving message");
            } else {
                let text = std::str::from_utf8(&buff.mtext).unwrap_or_else(|_| "Invalid UTF-8");
                println!("Child process read from msg: {}, {}", text, buff.number);
            }
        }
    } else {
        eprintln!("Fork failed");
    }
}
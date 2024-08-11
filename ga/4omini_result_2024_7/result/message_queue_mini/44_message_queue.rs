use libc::{c_int, MSG_NOERROR, IPC_PRIVATE, shmget, msgsnd, msgrcv};
use std::io::{self, Write}; // Needed for flush
use std::ptr;
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::os::unix::io::AsRawFd;
use std::mem;

const MY_TYPE: c_int = 9527;

#[repr(C)] // Represents the message buffer as a C struct
struct MsgBuf {
    mtype: c_long,
    mtext: [i8; 100], // mtext of size 100
    number: c_int,
}

fn main() {
    // Create a message queue
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };
    if msgid == -1 {
        eprintln!("msgget failed");
        exit(1);
    }

    let pid = unsafe { libc::fork() };
    
    if pid > 0 {
        // Parent process
        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep for 1 second

        let mut buff = MsgBuf {
            mtype: MY_TYPE.into(),
            mtext: [0; 100],
            number: 0,
        };

        // Get user input for mtext
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");
        let input_cstring = std::ffi::CString::new(input_string.trim()).unwrap();
        buff.mtext.copy_from_slice(input_cstring.as_bytes_with_nul());

        // Get user input for number
        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");
        buff.number = input_string.trim().parse().unwrap_or(0);

        // Send message
        if unsafe { msgsnd(msgid, &buff as *const _ as *const _, 
            mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), 
            0) } == -1 {
            eprintln!("msgsnd failed");
            exit(1);
        }

        // Wait for child process
        let _ = unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let mut buff = MsgBuf {
            mtype: 0, // This will be filled by the msgrcv call
            mtext: [0; 100],
            number: 0,
        };

        // Receive message
        if unsafe { msgrcv(msgid, &mut buff as *mut _ as *mut _, 
            mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), MY_TYPE, 0) } == -1 {
            eprintln!("msgrcv failed");
            exit(1);
        }

        // Print the received message
        let received_string = unsafe { std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy() };
        println!("Child process read from msg: {}, {}", received_string, buff.number);
    } else {
        eprintln!("fork failed");
    }
}
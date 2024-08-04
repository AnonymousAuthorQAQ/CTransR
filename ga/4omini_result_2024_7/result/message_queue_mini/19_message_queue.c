use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::process::{Command, exit};
use std::ptr;
use std::ffi::CString;

const MY_TYPE: i32 = 9527;

#[link(name = "c")]
extern "C" {
    fn msgget(key: i32, msgflg: i32) -> i32;
    fn msgsnd(msgid: i32, msgp: *const MsgBuf, msgsz: usize, msgflg: i32) -> i32;
    fn msgrcv(msgid: i32, msgp: *mut MsgBuf, msgsz: usize, msgtyp: i32, msgflg: i32) -> i32;
}

const IPC_CREAT: i32 = 512;
const IPC_EXCL: i32 = 512;

#[repr(C)]
struct MsgBuf {
    mtype: i32,
    mtext: [i8; 100],
    number: i32,
}

fn main() {
    let key = 0;  // Use a private key;
    let msgflg = IPC_CREAT | 0o666;  // create mode
    
    // Create the message queue
    let msgid = unsafe { msgget(key, msgflg) };
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    // Fork the process
    let pid = unsafe { libc::fork() };
    if pid > 0 {
        // In parent process
        let mut buff: MsgBuf = unsafe { std::mem::zeroed() };
        buff.mtype = MY_TYPE;

        // Input from user
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        let c_str = CString::new(input_text.trim()).unwrap();
        for (i, &b) in c_str.as_bytes().iter().enumerate() {
            buff.mtext[i] = b as i8;
        }

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        let mut number_str = String::new();
        io::stdin().read_line(&mut number_str).unwrap();
        buff.number = number_str.trim().parse().unwrap();

        // Send the message
        if unsafe { msgsnd(msgid, &buff as *const _ as *const _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i32>(), 0) } == -1 {
            eprintln!("msgsnd failed");
            exit(1);
        }

        // Wait for the child process to finish
        let _ = unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        // In child process
        let mut buff: MsgBuf = unsafe { std::mem::zeroed() };

        // Receive the message
        if unsafe { msgrcv(msgid, &mut buff as *mut _ as *mut _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i32>(), MY_TYPE, 0) } == -1 {
            eprintln!("msgrcv failed");
            exit(1);
        }

        // Convert the mtext back to String
        let message = unsafe { CString::from_raw(buff.mtext.as_ptr()).into_string().unwrap() };
        println!("Child process read from msg: {}, {}", message, buff.number);
    } else {
        eprintln!("fork failed");
    }
}
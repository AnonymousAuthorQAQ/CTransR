use libc::{c_char, c_int, c_long, size_t, ssize_t};
use std::ffi::{CStr, CString};
use std::ptr;
use std::process::Command;
use std::os::unix::process::CommandExt;

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}

extern "C" {
    fn msgget(key: c_int, msgflg: c_int) -> c_int;
    fn msgsnd(msqid: c_int, msgp: *const MsgBuf, msgsz: size_t, msgflg: c_int) -> c_int;
    fn msgrcv(msqid: c_int, msgp: *mut MsgBuf, msgsz: size_t, msgtype: c_long, msgflg: c_int) -> ssize_t;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
}

fn main() {
    unsafe {
        // Create message queue
        let msgid = msgget(IPC_PRIVATE, 0o666 | IPC_EXCL);
        if msgid == -1 {
            eprintln!("msgget failed");
            return;
        }

        let pid = fork();
        if pid > 0 {
            // Parent process
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Read from standard input
            println!("Please enter a string you want to send:");
            std::io::stdin().read_line(&mut buff.mtext[..])
                .expect("Failed to read line");

            // Read a number from standard input
            println!("Please enter a number you want to send:");
            let mut number_input = String::new();
            std::io::stdin().read_line(&mut number_input)
                .expect("Failed to read line");
            buff.number = number_input.trim().parse().expect("Please enter a valid number");

            // Send the message
            if msgsnd(msgid, &buff as *const _ as *const libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), 0) == -1 {
                eprintln!("msgsnd failed");
            }

            let mut status: c_int = 0;
            waitpid(pid, &mut status, 0);
        } else if pid == 0 {
            // Child process
            let mut buff = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            // Wait for message
            if msgrcv(msgid, &mut buff as *mut _ as *mut libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), MY_TYPE, 0) == -1 {
                eprintln!("msgrcv failed");
            } else {
                let msg_text = CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy();
                println!("Child process read from msg: {}, {}", msg_text, buff.number);
            }
        } else {
            eprintln!("fork failed");
        }
    }
}
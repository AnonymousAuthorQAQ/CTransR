use std::os::unix::process::CommandExt;
use std::ptr;
use std::ffi::{CString, CStr};
use std::io::{self, Write};
use std::mem;

use libc::{c_int, c_long, msgrcv, msgsnd, ftok, msgget, key_t, IPC_CREAT, IPC_EXCL, IPC_NOWAIT};

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: c_int,
}

const MY_TYPE: c_long = 9527;

fn main() {
    let key = unsafe { ftok(CString::new(".").unwrap().as_ptr(), 1) };
    let msgid = unsafe { msgget(key, IPC_CREAT | IPC_EXCL | 0o666) };
    if msgid == -1 {
        panic!("Failed to create message queue");
    }

    unsafe {
        let pid = libc::fork();
    
        match pid {
            -1 => panic!("Fork failed"),
            0 => {
                let mut buf: MsgBuf = mem::zeroed();
                msgrcv(msgid, &mut buf as *mut MsgBuf as *mut _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), MY_TYPE, 0);
                let string_received = CStr::from_ptr(buf.mtext.as_ptr() as *const _).to_str().expect("Failed to convert CStr to str");
                println!("Child process read from msg: {}, {}", string_received, buf.number);
            },
            _ => {
                let mut buf: MsgBuf = mem::zeroed();
                println!("Please enter a string you want to send:");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let string_to_send = CString::new(input.trim()).expect("Failed to convert input to CString");
                buf.mtype = MY_TYPE;
                buf.number = 0;
                buf.mtext[..string_to_send.to_bytes().len()].copy_from_slice(&string_to_send.to_bytes());
                msgsnd(msgid, &buf as *const MsgBuf as *const _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), IPC_NOWAIT);
                
                // Wait for child process to finish
                waitpid(pid, ptr::null_mut(), 0);
            },
        }
    }
}

fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int {
    unsafe {
        libc::waitpid(pid, status, options)
    }
}
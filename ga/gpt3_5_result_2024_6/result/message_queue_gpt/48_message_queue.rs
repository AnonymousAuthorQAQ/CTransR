use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::process::{Command, Child};
use std::mem;
use libc::{msgget, msgsnd, msgrcv, fork, getpid, waitpid, IPC_CREAT, IPC_EXCL, MSG_NOERROR, SIGCHLD};
use libc::{c_long, c_void};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: i32,
}

fn main() {
    unsafe {
        let msgid = msgget(libc::IPC_PRIVATE, 0o666 | IPC_EXCL);

        if msgid == -1 {
            perror("msgget");
            return;
        }

        let pid = fork();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let c_input = CString::new(input).unwrap();
            libc::strcpy(buff.mtext.as_mut_ptr(), c_input.as_ptr());

            let mut number = String::new();
            std::io::stdin().read_line(&mut number).unwrap();
            let number = number.trim().parse().unwrap();
            buff.number = number;

            msgsnd(msgid, &buff as *const MsgBuf as *const c_void, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), 0);

            waitpid(pid, std::ptr::null_mut(), 0);
        } else if pid == 0 {
            let mut buff: MsgBuf = std::mem::zeroed();
            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut c_void, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), MY_TYPE, MSG_NOERROR);
            println!("Child process read from msg: {}, {}", CStr::from_ptr(buff.mtext.as_ptr()).to_str().unwrap(), buff.number);
        } else {
            perror("fork");
        }
    }
}

fn perror(msg: &str) {
    unsafe {
        let c_msg = CString::new(msg).unwrap();
        libc::perror(c_msg.as_ptr());
    }
}
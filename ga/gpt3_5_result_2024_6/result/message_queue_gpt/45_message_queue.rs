extern crate libc;

use std::ffi::CString;
use std::mem;
use libc::{key_t, c_void, c_long, c_int, fork, getpid};
use std::process::Command;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_char;

const MY_TYPE: c_long = 9527;

#[repr(C)]
pub struct Msgbuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}

fn main() {
    unsafe {
        let msgid = libc::msgget(libc::IPC_PRIVATE, 0o666);
        if msgid == -1 {
            perror("msgget");
            return;
        }

        let pid = fork();
        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff: Msgbuf = Msgbuf {
                mtype: MY_TYPE,
                mtext: ['\0' as c_char; 100],
                number: 0,
            };

            // println!("Please enter a string you want to send:");
            std::io::stdin().read_line(&mut buff.mtext);

            // println!("Please enter a nubmer you want to send:");
            let mut number_str = String::new();
            std::io::stdin().read_line(&mut number_str);
            buff.number = number_str.trim().parse().unwrap();

            libc::msgsnd(msgid, &buff as *const Msgbuf as *const c_void, std::mem::size_of::<Msgbuf>() - std::mem::size_of::<bondhton.mtype>, 0);

            Command::new("waitpid")
                .arg(format!("{}",pid))
                .output()
                .expect("failed to execute process");
        } else if pid == 0 {
            // println!("Child process is waiting for msg:");
            let mut buff: Msgbuf = Msgbuf {
                mtype: 0,
                mtext: ['\0' as c_char; 100],
                number: 0,
            };

            libc::msgrcv(msgid, &buff as *mut Msgbuf as *mut c_void, std::mem::size_of::<Msgbuf>() - std::mem::size_of::<buff.mtype>, MY_TYPE, 0);
            println!("Child process read from msg: {}, {}", CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy(), buff.number);
        } else {
            perror("fork");
        }
    }
}

fn perror(msg: &str) {
    unsafe {
        libc::perror(CString::new(msg).unwrap().as_ptr());
    }
}
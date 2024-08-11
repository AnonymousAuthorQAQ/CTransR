use nix::sys::msg::{msgget, msgsnd, msgrcv, Msgflag, MsgType, MsgQueueAttr, MsgctlCmd, msqid_ds};
use nix::sys::wait::{waitpid, WaitpidFlag, WaitStatus};
use nix::unistd::{fork, ForkResult, getpid};

use std::io::{Read, Write};
use std::ptr;
use std::ffi::CString;
use std::collections::HashMap;

fn main() {
    let key = CString::new("key").expect("CString::new failed");
    let msgid = msgget(key, nix::libc::IPC_CREAT | 0o666).unwrap();

    match fork().expect("Fork failed") {
        ForkResult::Parent { child, .. } => {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut message = HashMap::new();
            let mut buff = MsgType {
                mtype: 1,
                un: message.as_mut_ptr()
            };

            print!("Please enter a string you want to send: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let text = CString::new(input.trim()).expect("CString::new failed");
            buff.un.message_buf = text.as_ptr() as *mut i64;

            println!("Please enter a number you want to send: ");
            let mut number = String::new();
            std::io::stdin().read_line(&mut number).unwrap();
            buff.un.message_buf = number.trim().parse::<i64>().unwrap() as *mut i64;

            msgsnd(msgid, &buff, std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype), Msgflag::empty()).unwrap();

            if let WaitStatus::Exited(pid, _code) = waitpid(child, None).unwrap() {
                println!("Child process with PID: {}, finished!", pid);
            }
        }

        ForkResult::Child => {
            let mut message = HashMap::new();
            let mut buff = MsgType {
                mtype: 1,
                un: message.as_ptr()
            };

            msgrcv(msgid, &mut buff, std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype) as i32,
                  Some(1), Msgflag::empty()).unwrap();

            let text = unsafe { std::ffi::CStr::from_ptr(buff.un.message_buf as *mut i8) };
            let number = unsafe { *buff.un.message_buf };

            println!("Child process read from msg: {}, {}", text.to_str().unwrap(), number);
            ()
        }
    }
}
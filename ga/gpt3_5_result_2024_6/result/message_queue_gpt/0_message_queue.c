use std::process::Command;
use std::env;
use std::ptr;
use std::ffi::CString;
use libc::{fork, waitpid, gets, scanf};

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let msgid = unsafe {
        libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL)
    };

    if msgid == -1 {
        panic!("Error creating message queue");
    }

    let pid = unsafe { fork() };

    match pid {
        0 => {
            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            println!("Child process is waiting for msg:");

            unsafe { libc::msgrcv(msgid, &mut buff as *mut MsgBuf as *mut libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), MY_TYPE, 0) };

            let text = std::str::from_utf8(&buff.mtext).unwrap();
            println!("Child process read from msg: {}, {}", text, buff.number);
        },
        _ if pid > 0 => {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            println!("Please enter a string you want to send:");
            unsafe { gets(buff.mtext.as_mut_ptr() as *mut i8) };

            println!("Please enter a number you want to send:");
            unsafe { scanf(b"%d\0".as_ptr(), &mut buff.number as *mut i32) };

            unsafe { libc::msgsnd(msgid, &mut buff as *mut MsgBuf as *const libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0) };

            let _ = Command::new("waitpid")
                .args(&[&pid.to_string()])
                .status();
        },
        _ => {
            println!("Error forking");
        }
    }
}
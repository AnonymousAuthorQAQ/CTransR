extern crate libc;

use libc::{c_int, c_long, size_t};
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;
use std::thread::sleep;
use std::time::Duration;

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [i8; 100],
    number: i32,
}

fn msgget(key: c_int, flags: c_int) -> c_int {
    unsafe { libc::msgget(key, flags) }
}

fn msgsnd(msgid: c_int, msgp: *const MsgBuf, msgsz: size_t, msgflg: c_int) -> c_int {
    unsafe { libc::msgsnd(msgid, msgp as *const libc::c_void, msgsz, msgflg) }
}

fn msgrcv(msgid: c_int, msgp: *mut MsgBuf, msgsz: size_t, msgtyp: c_long, msgflg: c_int) -> c_int {
    unsafe { libc::msgrcv(msgid, msgp as *mut libc::c_void, msgsz, msgtyp, msgflg) }
}

fn main() {
    let msgid = unsafe { msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL) };
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    let pid = unsafe { libc::fork() };
    if pid > 0 {
        // Parent process
        sleep(Duration::from_secs(1));

        let mut buff: MsgBuf = unsafe { mem::zeroed() };
        buff.mtype = MY_TYPE;

        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let c_string = CString::new(input.trim()).unwrap();
        buff.mtext[..c_string.as_bytes().len()].copy_from_slice(c_string.as_bytes());

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input).unwrap();
        buff.number = number_input.trim().parse().unwrap();

        msgsnd(msgid, &buff, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), 0);

        unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let mut buff: MsgBuf = unsafe { mem::zeroed() };
        msgrcv(msgid, &mut buff, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), MY_TYPE, 0);
        let message = unsafe { CStr::from_ptr(buff.mtext.as_ptr()) };
        println!(
            "Child process read from msg: {}, {}",
            message.to_string_lossy(),
            buff.number
        );
    } else {
        eprintln!("fork failed");
    }
}
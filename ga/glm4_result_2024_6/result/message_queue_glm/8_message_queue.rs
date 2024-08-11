use std::io::{self, Read};
use std::process::Command;
use std::ptr::null;
use std::str;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn msgget(key: c_int, msgflg: c_int) -> c_int;
    fn msgsnd(msgid: c_int, msgp: *const ::std::os::raw::c_void, msgsz: usize, msgflg: c_int) -> c_int;
    fn msgrcv(msgid: c_int, msgp: *mut ::std::os::raw::c_void, msgsz: usize, msgtyp: c_long, msgflg: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
}

const MY_TYPE: c_long = 9527;
const IPC_PRIVATE: c_int = 0;
const IPC_EXCL: c_int = 0o20;
const MSG_SIZE: usize = 100;

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0666 | IPC_EXCL) };
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    let pid = unsafe { fork() };
    if pid > 0 {
        // Parent
        unsafe {
            waitpid(pid, null_mut(), 0);
        }
    } else if pid == 0 {
        // Child
        let mut buffer = [0; MSG_SIZE];
        let mut msgp = buffer.as_mut_ptr();
        let msgsz = buffer.len();
        let msgtyp = MY_TYPE;

        let result = unsafe { msgrcv(msgid, msgp, msgsz as usize, msgtyp, 0) };
        if result == -1 {
            eprintln!("msgrcv failed");
            return;
        }

        let c_str = unsafe { CStr::from_ptr(msgp as *const c_char) };
        let str_slice = c_str.to_str().unwrap();
        let number = unsafe { *(msgp as *const i32) };

        println!("Child process read from msg: {}, {}", str_slice, number);
    } else {
        eprintln!("fork failed");
    }
}
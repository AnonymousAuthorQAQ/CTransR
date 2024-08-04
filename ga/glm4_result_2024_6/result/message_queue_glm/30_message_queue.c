use std::io::{self, Read};
use std::process::exit;
use std::ptr;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ffi::CStr;

extern "C" {
    fn msgget(key: c_int, flag: c_int) -> c_int;
    fn msgsnd(msgid: c_int, msgp: *const ::std::os::raw::c_void, msgsz: size_t, flag: c_int) -> c_int;
    fn msgrcv(msgid: c_int, msgp: *mut ::std::os::raw::c_void, msgsz: size_t, msgtyp: c_long, flag: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn perror(s: *const c_char);
}

const MY_TYPE: c_long = 9527;

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0666 | IPC_EXCL) };

    if msgid == -1 {
        unsafe { perror(CString::new("msgget").unwrap().as_ptr()) };
        exit(1);
    }

    let pid = unsafe { fork() };

    if pid > 0 {
        unsafe { sleep(1) };

        let mut buff = Message { mtype: MY_TYPE, mtext: [0; 100], number: 0 };

        io::stdin().read_line(&mut buff.mtext).expect("Failed to read line");
        buff.mtext.truncate(buff.mtext.len() - 1); // Remove newline

        io::stdin().read_line(&mut buff.number).expect("Failed to read number");
        buff.number = buff.number.trim().parse().expect("Failed to parse number");

        let result = unsafe {
            msgsnd(msgid, &buff as *const _ as *const _, 
                   std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype), 0)
        };

        if result == -1 {
            unsafe { perror(CString::new("msgsnd").unwrap().as_ptr()) };
            exit(1);
        }

        unsafe { waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        let mut buff = Message { mtype: MY_TYPE, mtext: [0; 100], number: 0 };

        let result = unsafe {
            msgrcv(msgid, &mut buff as *mut _ as *mut _, 
                    std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype), MY_TYPE, 0)
        };

        if result == -1 {
            unsafe { perror(CString::new("msgrcv").unwrap().as_ptr()) };
            exit(1);
        }

        println!("Child process read from msg: {}", buff.mtext);
        println!("Child process read from msg: {}", buff.number);
    } else {
        unsafe { perror(CString::new("fork").unwrap().as_ptr()) };
        exit(1);
    }
}

#[repr(C)]
struct Message {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}
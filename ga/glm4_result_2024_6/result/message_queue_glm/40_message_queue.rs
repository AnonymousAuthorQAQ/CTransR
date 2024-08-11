use std::io::{self, Read};
use std::process::Command;
use std::ptr;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::process::CommandExt;
use std::ffi::CStr;

extern "C" {
    fn msgget(key: c_int, flag: c_int) -> c_int;
    fn msgsnd(msgid: c_int, msgp: *const std::os::raw::c_void, msgsz: size_t, flag: c_int) -> c_int;
    fn msgrcv(msgid: c_int, msgp: *mut std::os::raw::c_void, msgsz: size_t, msgtyp: c_long, flag: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn getpid() -> c_int;
}

fn main() {
    unsafe {
        let key = 0; // IPC_PRIVATE
        let flag = 0o666 | 0o200; // IPC_EXCL | 0666

        let msgid = msgget(key, flag);
        if msgid == -1 {
            eprintln!("msgget failed");
            return;
        }

        let pid = fork();
        if pid > 0 {
            // Parent
            std::thread::sleep_ms(1000);

            let mut buff = std::mem::zeroed();
            buff.mtype = 9527; // MY_TYPE

            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let c_str = CString::new(input.trim()).unwrap();
            ptr::copy_nonoverlapping(c_str.as_ptr(), &mut buff.mtext as *mut _ as *mut c_char, 100);

            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            let mut num = String::new();
            io::stdin().read_line(&mut num).unwrap();
            buff.number = num.trim().parse().unwrap();

            if msgsnd(msgid, &buff as *const _ as *const std::os::raw::c_void, 100 + 4, 0) == -1 {
                eprintln!("msgsnd failed");
            }

            let status = std::mem::zeroed();
            if waitpid(pid, &status, 0) == -1 {
                eprintln!("waitpid failed");
            }
        } else if pid == 0 {
            // Child
            let mut buff = std::mem::zeroed();
            if msgrcv(msgid, &mut buff as *mut _ as *mut std::os::raw::c_void, 100 + 4, 9527, 0) == -1 {
                eprintln!("msgrcv failed");
            }

            let text = CStr::from_ptr(buff.mtext as *const c_char).to_string_lossy().into_owned();
            println!("Child process read from msg: {}, {}", text, buff.number);
        } else {
            eprintln!("fork failed");
        }
    }
}
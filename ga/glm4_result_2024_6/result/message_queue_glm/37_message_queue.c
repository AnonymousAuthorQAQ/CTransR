use std::io::{self, Read};
use std::process::wait;
use std::ptr;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::unix::process::CommandExt;
use std::os::unix::io::{FromRawFd, RawFd};
use std::ptr::null_mut;
use std::ffi::CStr;

extern "C" {
    fn msgget(key: c_int, flags: c_int) -> c_int;
    fn msgctl(msgqid: c_int, cmd: c_int, buf: *mut c_char) -> c_int;
    fn msgsnd(msgqid: c_int, msgp: *const c_char, msgsz: c_int, flags: c_int) -> c_int;
    fn msgrcv(msgqid: c_int, msgp: *mut c_char, msgsz: c_int, msgtyp: c_long, flags: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn perror(s: *const c_char);
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0666 | IPC_EXCL) };
    if msgid == -1 {
        unsafe { perror(CString::new("msgget").unwrap().as_ptr()) };
        return;
    }

    let pid = unsafe { fork() };
    if pid > 0 {
        unsafe { waitpid(pid, null_mut(), 0) };
    } else if pid == 0 {
        let mut buff = CString::new("".to_string()).unwrap();
        let mut number: c_int = 0;
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut buff).unwrap();
        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut buff).unwrap();
        number = buff.parse().unwrap();

        unsafe {
            msgsnd(msgid, buff.as_ptr() as *const c_char, buff.as_bytes().len() as c_int, 0);
        }
    } else {
        unsafe { perror(CString::new("fork").unwrap().as_ptr()) };
    }
}
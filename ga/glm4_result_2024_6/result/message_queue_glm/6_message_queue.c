use std::io::{self, Read};
use std::process::exit;
use std::ptr;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr::null_mut;
use std::str;

extern "C" {
    fn msgget(key: c_int, flags: c_int) -> c_int;
    fn msgctl(msgqid: c_int, cmd: c_int, buf: *mut c_char) -> c_int;
    fn msgsnd(msgqid: c_int, msgp: *const c_char, msgsz: c_int, flags: c_int) -> c_int;
    fn msgrcv(msgqid: c_int, msgp: *mut c_char, msgsz: c_int, msgtyp: c_long, flags: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn perror(s: *const c_char);
}

const MY_TYPE: c_long = 9527;
const IPC_PRIVATE: c_int = 0;
const IPC_EXCL: c_int = 0o20;
const IPC_RMID: c_int = 0;
const MSGSZ: usize = 100;

fn main() {
    let mut msgid: c_int = unsafe { msgget(IPC_PRIVATE, 0666 | IPC_EXCL) };
    if msgid == -1 {
        unsafe { perror(null_mut()) };
        exit(1);
    }

    let pid: c_int = unsafe { fork() };
    if pid > 0 {
        unsafe {
            msgctl(msgid, IPC_RMID, null_mut());
            msgid = msgget(IPC_PRIVATE, 0666 | IPC_EXCL);
        }

        if msgid == -1 {
            unsafe { perror(null_mut()) };
            exit(1);
        }

        let mut buff = CString::new("".to_string()).unwrap();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut buff).unwrap();
        println!("Please enter a number you want to send:");
        let mut number: i32 = 0;
        io::stdin().read_line(&mut number).unwrap();

        let mut full_msg = CString::new(format!("{} {}", buff, number)).unwrap();
        let full_msg_ptr = full_msg.as_ptr();
        let full_msg_len = full_msg.to_bytes_with_nul().len() as c_int;

        unsafe {
            msgsnd(msgid, full_msg_ptr, full_msg_len, 0);
            waitpid(pid, null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff = CString::new("".to_string()).unwrap();
        let mut buff_ptr = buff.as_ptr();
        let mut buff_len: c_int = 0;

        loop {
            unsafe {
                buff_len = msgrcv(msgid, buff_ptr, MSGSZ as c_int, MY_TYPE, 0);
            }

            if buff_len == -1 {
                unsafe { perror(null_mut()) };
                exit(1);
            } else {
                break;
            }
        }

        let msg_str = unsafe { str::from_utf8_unchecked(buff.to_bytes_with_nul()) };
        println!("Child process read from msg: {}", msg_str);
    } else {
        unsafe { perror(null_mut()) };
        exit(1);
    }
}
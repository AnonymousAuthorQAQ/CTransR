use std::io::{self, Read};
use std::process::exit;
use std::ptr::null;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::unix::process::CommandExt;
use std::ptr;
use std::str;
use std::ffi::CStr;

extern "C" {
    fn msgget(key: c_int, flag: c_int) -> c_int;
    fn msgctl(msgqid: c_int, cmd: c_int, buf: *mut c_char) -> c_int;
    fn msgsnd(msgqid: c_int, msgp: *const c_char, msgsz: c_int, flag: c_int) -> c_int;
    fn msgrcv(msgqid: c_int, msgp: *mut c_char, msgsz: c_int, msgtyp: c_long, flag: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn getpid() -> c_int;
    fn perror(s: *const c_char);
}

const MY_TYPE: c_long = 9527;
const IPC_PRIVATE: c_int = 0;
const IPC_EXCL: c_int = 0o20;
const IPC_RMID: c_int = 0;
const MSGSZ: usize = 100;

#[repr(C)]
struct Msgbuf {
    mtype: c_long,
    mtext: [c_char; MSGSZ],
    number: c_int,
}

fn main() {
    let mut msgid: c_int = unsafe { msgget(IPC_PRIVATE, 0666 | IPC_EXCL) };
    if msgid == -1 {
        unsafe { perror(null()) };
        exit(1);
    }

    let pid = unsafe { fork() };
    if pid > 0 {
        unsafe { sleep(1) };

        let mut buff = Msgbuf {
            mtype: MY_TYPE,
            mtext: [0; MSGSZ],
            number: 0,
        };

        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut buff.mtext).unwrap();
        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut buff.number).unwrap();

        unsafe {
            msgsnd(msgid, &buff as *const _ as *const c_char, (std::mem::size_of::<Msgbuf>() - std::mem::size_of::<c_long>()) as c_int, 0);
        }

        let status: c_int = 0;
        unsafe { waitpid(pid, &status, 0) };
    } else if pid == 0 {
        let mut buff = Msgbuf {
            mtype: MY_TYPE,
            mtext: [0; MSGSZ],
            number: 0,
        };

        unsafe {
            msgrcv(msgid, &mut buff as *mut _ as *mut c_char, (std::mem::size_of::<Msgbuf>() - std::mem::size_of::<c_long>()) as c_int, MY_TYPE, 0);
        }

        let text = unsafe { CStr::from_ptr(buff.mtext.as_ptr()) }.to_string_lossy().into_owned();
        println!("Child process read from msg: {}, {}", text, buff.number);
    } else {
        unsafe { perror(null()) };
    }

    unsafe {
        msgctl(msgid, IPC_RMID, null());
    }
}
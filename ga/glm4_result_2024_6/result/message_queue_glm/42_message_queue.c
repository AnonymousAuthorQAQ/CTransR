use std::io::{self, Write};
use std::process::wait;
use std::ptr::null;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::unix::process::CommandExt;
use std::ptr;

extern "C" {
    fn msgget(key: c_int, flag: c_int) -> c_int;
    fn msgctl(msqid: c_int, cmd: c_int, buf: *mut c_char) -> c_int;
    fn msgsnd(msqid: c_int, msgp: *const c_char, msgsz: size_t, flag: c_int) -> c_int;
    fn msgrcv(msqid: c_int, msgp: *mut c_char, msgsz: size_t, mtype: c_long, flag: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn perror(s: *const c_char);
}

const MY_TYPE: c_long = 9527;

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0666 | IPC_EXCL) };
    if msgid == -1 {
        unsafe { perror(null()) };
        return;
    }

    let pid = unsafe { fork() };
    if pid > 0 {
        unsafe { sleep(1) };

        let mut buff: [i8; 100] = [0; 100];
        print!("Please enter a string you want to send:\n");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buff).unwrap();
        print!("Please enter a number you want to send:\n");
        io::stdout().flush().unwrap();
        let number: i32 = io::stdin().read_line().unwrap().trim().parse().unwrap();

        let mut msg = CString::new(buff).unwrap();
        msg.push('\0');
        unsafe {
            msgsnd(msgid, msg.as_ptr(), msg.to_bytes_with_nul().len() as usize, 0);
        }

        unsafe { waitpid(pid, null_mut(), 0) };
    } else if pid == 0 {
        let mut buff: [i8; 100] = [0; 100];
        unsafe {
            msgrcv(msgid, buff.as_mut_ptr() as *mut i8, buff.len() as usize, MY_TYPE, 0);
        }
        println!("Child process read from msg: {}", unsafe {
            CStr::from_ptr(buff.as_ptr()).to_string_lossy().into_owned()
        });
    } else {
        unsafe { perror(null()) };
    }

    unsafe {
        msgctl(msgid, IPC_RMID, null_mut());
    }
}
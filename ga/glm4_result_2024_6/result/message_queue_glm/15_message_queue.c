use std::io::{self, Read};
use std::process::wait;
use std::ptr::null;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

extern "C" {
    fn msgget(key: c_int, flags: c_int) -> c_int;
    fn msgsnd(msgid: c_int, msgp: *const ::std::os::raw::c_void, msgsz: size_t, flags: c_int) -> c_int;
    fn msgrcv(msgid: c_int, msgp: *mut ::std::os::raw::c_void, msgsz: size_t, msgtyp: c_long, flags: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn sleep(seconds: c_uint);
}

const MY_TYPE: c_long = 9527;

fn main() {
    let msgid = unsafe { msgget(0, 0o666 | 0o200) };
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    let pid = unsafe { fork() };
    if pid > 0 {
        unsafe { sleep(1) };

        let mut buff = Box::new([0; 100]);
        print!("Please enter a string you want to send:\n");
        io::stdin().read_line(&mut buff).expect("Failed to read line");
        print!("Please enter a number you want to send:\n");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Invalid number");

        let c_str = CString::new(buff).expect("CString::new failed");
        let mut msg = Box::new([0; 104]);
        msg[0] = MY_TYPE;
        unsafe {
            ptr::copy_nonoverlapping(c_str.as_ptr(), msg[1..].as_mut_ptr(), 100);
            msg[101] = number;
        }

        if unsafe { msgsnd(msgid, msg.as_ptr() as *const _, 104, 0) } == -1 {
            eprintln!("msgsnd failed");
        }

        unsafe { waitpid(pid, null_mut(), 0) };
    } else if pid == 0 {
        let mut msg = Box::new([0; 104]);
        if unsafe { msgrcv(msgid, msg.as_mut_ptr() as *mut _, 104, MY_TYPE, 0) } == -1 {
            eprintln!("msgrcv failed");
            return;
        }

        let c_str = unsafe { CStr::from_ptr(msg[1..].as_ptr() as *const c_char).to_str().expect("Invalid string") };
        println!("Child process read from msg: {}, {}", c_str, msg[101]);
    } else {
        eprintln!("fork failed");
    }
}
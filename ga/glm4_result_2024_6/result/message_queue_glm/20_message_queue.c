use std::io::{self, Read};
use std::process::Command;
use std::ptr;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::unix::process::CommandExt;

extern "C" {
    fn msgget(key: c_int, flags: c_int) -> c_int;
    fn msgsnd(msgid: c_int, msgp: *const ::std::os::raw::c_void, msgsz: size_t, msgflg: c_int) -> c_int;
    fn msgrcv(msgid: c_int, msgp: *mut ::std::os::raw::c_void, msgsz: size_t, msgtyp: c_long, msgflg: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
}

const MY_TYPE: c_long = 9527;

fn main() {
    let key = 0;
    let flags = 0666 | 0o1000; // IPC_CREAT | IPC_EXCL

    let msgid = unsafe { msgget(key, flags) };
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    let pid = unsafe { fork() };
    if pid > 0 {
        // Parent
        std::thread::sleep_ms(1000);

        let mut input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        let mut number = String::new();
        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Failed to parse number");

        let mut msg = CString::new(input).expect("CString::new failed");
        let msg_size = (msg.as_bytes_with_nul().len() + std::mem::size_of::<i32>()) as usize;

        let mut msgp = vec![0; msg_size];
        unsafe {
            ptr::copy_nonoverlapping(msg.as_ptr(), msgp.as_mut_ptr(), msg.as_bytes_with_nul().len());
            ptr::copy_nonoverlapping(&number as *const i32, msgp.as_mut_ptr() as *mut i32, std::mem::size_of::<i32>());
        }

        let msgflg = 0;
        if unsafe { msgsnd(msgid, msgp.as_ptr() as *const ::std::os::raw::c_void, msg_size as size_t, msgflg) } == -1 {
            eprintln!("msgsnd failed");
            return;
        }

        let status = 0;
        if unsafe { waitpid(pid, &status, 0) } == -1 {
            eprintln!("waitpid failed");
            return;
        }
    } else if pid == 0 {
        // Child
        let msg_size = 100 + std::mem::size_of::<i32>();
        let mut msgp = vec![0; msg_size];

        let msgflg = 0;
        if unsafe { msgrcv(msgid, msgp.as_mut_ptr() as *mut ::std::os::raw::c_void, msg_size as size_t, MY_TYPE, msgflg) } == -1 {
            eprintln!("msgrcv failed");
            return;
        }

        let text = unsafe { CStr::from_ptr(msgp.as_ptr() as *const c_char).to_string_lossy().into_owned() };
        let number = unsafe { *(msgp.as_ptr() as *const i32) };

        println!("Child process read from msg: {}, {}", text, number);
    } else {
        eprintln!("fork failed");
    }
}
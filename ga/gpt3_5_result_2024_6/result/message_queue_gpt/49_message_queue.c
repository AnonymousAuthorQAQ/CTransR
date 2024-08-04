use std::mem;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ptr;
use std::process;

use libc::{msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL};

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [c_char; 100],
    number: c_int,
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error in msgget");
        process::exit(-1);
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff: MsgBuf = unsafe { mem::zeroed() };

        buff.mtype = MY_TYPE;

        println!("Please enter a string you want to send:");
        let mut input_str = String::new();
        std::io::stdin().read_line(&mut input_str).expect("Failed to read line");
        let c_str = CString::new(input_str.trim()).expect("CString::new failed");
        let c_bytes = c_str.as_bytes_with_nul();
        unsafe {
            ptr::copy_nonoverlapping(c_bytes.as_ptr(), buff.mtext.as_mut_ptr() as *mut u8, c_bytes.len());
        }

        println!("Please enter a number you want to send:");
        let mut input_num = String::new();
        std::io::stdin().read_line(&mut input_num).expect("Failed to read line");
        let num: i32 = input_num.trim().parse().expect("Please enter a valid number");
        buff.number = num;

        unsafe {
            msgsnd(msgid, &buff as *const MsgBuf as *const std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0);
        }

        unsafe {
            libc::waitpid(pid, std::ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        // Child process
        let mut buff: MsgBuf = unsafe { mem::zeroed() };

        println!("Child process is waiting for msg:");
        unsafe {
            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), MY_TYPE, 0);
        }

        let c_str = unsafe { std::ffi::CStr::from_ptr(buff.mtext.as_ptr()) };
        let str_slice = c_str.to_str().expect("Failed to convert C string to Rust string");

        println!("Child process read from msg: {}, {}", str_slice, buff.number);
    } else {
        eprintln!("Error in fork");
    }
}
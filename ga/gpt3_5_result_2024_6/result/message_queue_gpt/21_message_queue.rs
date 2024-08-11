use std::ffi::{CString, CStr};
use std::os::raw::{c_long, c_int};
use std::mem;
use std::ptr;
use std::process;

use libc::{msgbuf, msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL, fork, waitpid, c_char};

const MY_TYPE: c_long = 9527;

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error in msgget");
        process::exit(-1);
    }

    unsafe {
        let pid = fork();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
        
            let mut buff: msgbuf = mem::zeroed();
            buff.mtype = MY_TYPE;
        
            let mut input_text = String::new();
            println!("Please enter a string you want to send:");
            std::io::stdin().read_line(&mut input_text).expect("Failed to read input");
            let c_input_text = CString::new(input_text.trim()).expect("Failed to convert to CString");
            ptr::copy(c_input_text.as_ptr(), buff.mtext.as_mut_ptr() as *mut c_char, c_input_text.as_bytes().len());
        
            println!("Please enter a number you want to send:");
            let mut input_number = String::new();
            std::io::stdin().read_line(&mut input_number).expect("Failed to read input");
            let number: i32 = input_number.trim().parse().expect("Failed to parse number");
            buff.number = number;
        
            msgsnd(msgid, &buff as *const msgbuf as *const _, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), 0);
        
            waitpid(pid, std::ptr::null_mut(), 0);
        } else if pid == 0 {
            let mut buff: msgbuf = mem::zeroed();
            msgrcv(msgid, &mut buff as *mut msgbuf as *mut _, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), MY_TYPE, 0);
        
            let received_text = CStr::from_ptr(buff.mtext.as_ptr() as *const c_char).to_string_lossy();
            println!("Child process read from msg: {}, {}", received_text, buff.number);
        } else {
            eprintln!("Error in fork");
        }
    }
}
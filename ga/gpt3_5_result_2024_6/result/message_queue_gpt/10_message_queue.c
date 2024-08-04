use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::mem;
use std::ptr;
use std::process;
use std::thread;
use libc::{c_long, c_int, msgrcv, msgsnd, IPC_PRIVATE, IPC_EXCL};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct msgbuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error creating message queue");
        process::exit(1);
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        thread::sleep(std::time::Duration::from_secs(1));

        let mut buff: msgbuf = unsafe { mem::zeroed() };

        buff.mtype = MY_TYPE;

        let mut input_text = String::new();
        println!("Please enter a string you want to send:");
        std::io::stdin().read_line(&mut input_text).unwrap();
        let input_text = CString::new(input_text.trim()).expect("CString::new failed");

        let input_number: c_int;
        println!("Please enter a number you want to send:");
        let mut input_number_str = String::new();
        std::io::stdin().read_line(&mut input_number_str).unwrap();
        input_number = input_number_str.trim().parse().unwrap();

        unsafe {
            ptr::copy_nonoverlapping(input_text.as_ptr(), buff.mtext.as_mut_ptr(), input_text.as_bytes().len());
        }
        buff.number = input_number;

        unsafe {
            msgsnd(msgid, &buff as *const msgbuf as *const std::ffi::c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), 0);
        }

        unsafe {
            libc::waitpid(pid, std::ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff: msgbuf = unsafe { mem::zeroed() };

        println!("Child process is waiting for msg:");
        unsafe {
            msgrcv(msgid, &mut buff as *mut msgbuf as *mut std::ffi::c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), MY_TYPE, 0);
        }
        
        let received_text = unsafe { CStr::from_ptr(buff.mtext.as_ptr()).to_str().unwrap() };
        println!("Child process read from msg: {}, {}", received_text, buff.number);
    } else {
        eprintln!("Error forking process");
    }
}
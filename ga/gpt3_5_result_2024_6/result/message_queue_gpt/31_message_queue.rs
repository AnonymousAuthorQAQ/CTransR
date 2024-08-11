use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::process::{Command, Stdio};
use std::io::{self, Write};

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [c_char; 100],
    number: i32,
}

fn main() {
    let mut msgid: i32;

    unsafe {
        msgid = libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL);
        if msgid == -1 {
            panic!("msgget failed");
        }
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff = MsgBuf {
            mtype: 9527,
            mtext: [0; 100],
            number: 0,
        };

        let mut input = String::new();
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let c_input = CString::new(input).unwrap();
        let input_ptr = c_input.as_ptr();
        unsafe {
            libc::strcpy(buff.mtext.as_mut_ptr(), input_ptr);
        }

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        let mut number = String::new();
        io::stdin().read_line(&mut number).unwrap();
        buff.number = number.trim().parse().unwrap();

        unsafe {
            libc::msgsnd(msgid, &buff as *const _ as *const std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0);
            libc::waitpid(pid, std::ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff = MsgBuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        unsafe {
            libc::msgrcv(msgid, &mut buff as *mut _ as *mut std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 9527, 0);
            let cstr = std::ffi::CStr::from_ptr(buff.mtext.as_ptr());
            let str_slice = cstr.to_str().unwrap();
            println!("Child process read from msg: {}, {}", str_slice, buff.number);
        }
    } else {
        eprintln!("Fork failed");
    }
}
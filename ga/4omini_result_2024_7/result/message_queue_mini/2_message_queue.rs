use libc::{c_int, pid_t, msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL, perror};
use std::ffi::CString;
use std::io::{self, Write};
use std::mem::size_of;
use std::os::unix::io::RawFd;
use std::process::{Command, exit};
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;
use std::os::unix::process::CommandExt;

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [i8; 100],
    number: i32,
}

fn main() {
    let msgid: c_int = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        unsafe {
            perror(CString::new("msgget").unwrap().as_ptr());
        }
        return;
    }

    let pid: pid_t = unsafe { libc::fork() };

    if pid > 0 {
        sleep(Duration::from_secs(1));

        let mut buff = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        print!("Please enter a string you want to send:\n");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let c_string = CString::new(input.trim()).unwrap();
        let bytes = c_string.as_bytes_with_nul();
        buff.mtext[..bytes.len()].copy_from_slice(bytes);

        print!("Please enter a number you want to send:\n");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        buff.number = input.trim().parse::<i32>().unwrap();

        unsafe {
            msgsnd(msgid, &buff as *const MsgBuf as *const _, size_of::<MsgBuf>() as c_int - size_of::<i64>() as c_int, 0);
        }

        // Wait for the child to finish
        let _ = unsafe { libc::waitpid(pid, null_mut(), 0) };
    } else if pid == 0 {
        let mut buff = MsgBuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        unsafe {
            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut _, size_of::<MsgBuf>() as c_int - size_of::<i64>() as c_int, MY_TYPE, 0);
        }

        let message = unsafe { std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy() };
        println!("Child process read from msg: {}, {}", message, buff.number);
    } else {
        unsafe {
            perror(CString::new("fork").unwrap().as_ptr());
        }
    }
}
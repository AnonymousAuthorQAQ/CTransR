use std::mem;
use std::os::raw::{c_char, c_int};
use std::ffi::{CString, CStr};
use std::ptr;
use libc::{c_long, fork, pid_t, sleep, waitpid, gets, scanf};

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}

const MY_TYPE: c_long = 9527;

fn main() {
    let msgid = unsafe { msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL) };
    if msgid == -1 {
        eprintln!("Error creating message queue");
        return;
    }

    let pid = unsafe { fork() };

    if pid > 0 {
        unsafe {
            sleep(1);
        }

        let mut buf: MsgBuf = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        println!("Please enter a string you want to send:");
        unsafe {
            gets(buf.mtext.as_mut_ptr());
            println!("Please enter a number you want to send:");
            scanf(b"%d\0".as_ptr() as *const i8, &mut buf.number);
            msgsnd(msgid, &mut buf, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>() as isize, 0);
            waitpid(pid, ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buf: MsgBuf = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };
        
        println!("Child process is waiting for msg:");
        unsafe {
            msgrcv(msgid, &mut buf, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>() as isize, MY_TYPE, 0);
            let c_str = CStr::from_ptr(buf.mtext.as_ptr());
            let str_buf = c_str.to_string_lossy();
            println!("Child process read from msg: {}, {}", str_buf, buf.number);
        }
    } else {
        eprintln!("Fork error");
    }
}
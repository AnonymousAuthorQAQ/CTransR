use std::ffi::CString;
use std::mem;
use std::ptr;
use std::process;
use std::thread;
use std::time::Duration;

use libc;

const MY_TYPE: i64 = 9527;

fn main() {
    let key = unsafe { libc::IPC_PRIVATE };
    let msgid = unsafe { libc::msgget(key, 0o666 | libc::IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error in msgget");
        process::exit(1);
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        thread::sleep(Duration::from_secs(1));

        let mut buff: libc::msgbuf = unsafe { mem::zeroed() };
        buff.mtype = MY_TYPE;
        let text = CString::new("Hello from parent").unwrap();
        let text_ptr = text.as_ptr();
        libc::strcpy(buff.mtext.as_mut_ptr(), text_ptr);
        buff.number = 123;

        unsafe {
            libc::msgsnd(
                msgid,
                &buff as *const _ as *const libc::c_void,
                mem::size_of::<libc::msgbuf>() - mem::size_of::<libc::c_long>(),
                0,
            );
        }

        unsafe {
            libc::waitpid(pid, ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff: libc::msgbuf = unsafe { mem::zeroed() };
        libc::msgrcv(
            msgid,
            &mut buff as *mut _ as *mut libc::c_void,
            mem::size_of::<libc::msgbuf>() - mem::size_of::<libc::c_long>(),
            MY_TYPE,
            0,
        );

        let received_text = std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_str().unwrap();
        println!("Child process read from msg: {}, {}", received_text, buff.number);
    } else {
        eprintln!("Error in fork");
    }
}
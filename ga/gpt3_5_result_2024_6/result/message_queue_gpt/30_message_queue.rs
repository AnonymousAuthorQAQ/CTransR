use std::ffi::CString;
use std::os::raw::c_char;

use libc::{msgbuf, msgget, msgsnd, msgrcv, fork, getpid};
use std::ptr;

const MY_TYPE: i64 = 9527;

fn main() {
    let msgid = unsafe { msgget(libc::IPC_PRIVATE, libc::IPC_EXCL | 0o666) };

    if msgid == -1 {
        panic!("Error creating message queue");
    }

    let pid = unsafe { fork() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff = msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        let input_text = "Hello from parent process";
        let input_number = 123;

        let c_input_text = CString::new(input_text).expect("Error converting to CString");
        let c_input_text_ptr = c_input_text.as_ptr() as *mut c_char;
        unsafe {
            std::ptr::copy(c_input_text_ptr, buff.mtext.as_mut_ptr(), input_text.len());

            buff.number = input_number;

            msgsnd(msgid, &buff as *const msgbuf as *const std::ffi::c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<i64>(), 0);
        }
    } else if pid == 0 {
        let mut buff = msgbuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        unsafe {
            msgrcv(msgid, &mut buff as *mut msgbuf as *mut std::ffi::c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<i64>(), MY_TYPE, 0);

            let received_text = std::ffi::CStr::from_ptr(buff.mtext.as_ptr() as *const c_char).to_str().expect("Error converting to str");
            let received_number = buff.number;

            println!("Child process read from msg: {}, {}", received_text, received_number);
        }
    } else {
        panic!("Error forking process");
    }
}
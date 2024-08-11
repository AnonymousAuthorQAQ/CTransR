use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ptr;
use std::mem;
use std::process::Command;
use std::process::Child;
use libc::msgget;
use libc::msgsnd;
use libc::msgrcv;
use libc::ftok;
use libc::IPC_CREAT;
use libc::IPC_EXCL;
use libc::c_void;
use libc::c_long;
use libc::c_uint;

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct msgbuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int,
}

fn main() {
    let key = unsafe { ftok(CString::new(".").unwrap().as_ptr() as *const c_char, 65) };
    let msgid = unsafe { msgget(key, IPC_CREAT | IPC_EXCL | 0o666) };

    let pid = unsafe { Command::new("sh").spawn().unwrap().id() as i32 };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff: msgbuf = unsafe { mem::zeroed() };
        buff.mtype = MY_TYPE;

        let input_text = String::from("Hello, from Rust!");
        let input_number: c_int = 123;

        let input_text_cstring = CString::new(input_text).unwrap();
        let input_text_ptr = input_text_cstring.into_raw();
        unsafe {
            ptr::copy_nonoverlapping(input_text_ptr as *const u8, buff.mtext.as_mut_ptr() as *mut u8, input_text.len());
        }

        buff.number = input_number;

        unsafe {
            msgsnd(msgid, &buff as *const msgbuf as *const c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), 0);
        }

        unsafe {
            Command::new("wait").args(&[&pid.to_string()]).output().unwrap();
        }
    } else if pid == 0 {
        let mut buff: msgbuf = unsafe { mem::zeroed() };
        buff.mtype = MY_TYPE;

        unsafe {
            msgrcv(msgid, &mut buff as *mut msgbuf as *mut c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), MY_TYPE, 0);
        }

        let received_text = unsafe { CString::from_raw(buff.mtext.as_mut_ptr()) };
        let received_text_str = received_text.to_str().unwrap();
        let received_number = buff.number;

        println!("Child process read from msg: {}, {}", received_text_str, received_number);
    }
}
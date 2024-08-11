use libc::{msgbuf};
use std::ptr;
use std::ffi::CString;
use std::mem;

fn main() {
    let mut msgid: i32;
    let mut pid: i32;

    let mut buff: msgbuf;

    unsafe {
        msgid = libc::msgget(libc::IPC_PRIVATE, 0o666);
        if msgid == -1 {
            panic!("Error in creating message queue");
        }

        pid = libc::fork();
        
        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            buff.mtype = 9527;
    
            let text = CString::new("Hello from Rust").unwrap();
            ptr::copy_nonoverlapping(text.as_ptr(), buff.mtext.as_mut_ptr() as *mut i8, text.as_bytes().len());

            let number = 42;
            buff.number = number;

            let res = libc::msgsnd(msgid, &buff as *const _ as *const libc::c_void, mem::size_of::<msgbuf>() - mem::size_of::<libc::c_long>(), 0);
            if res == -1 {
                panic!("Error in sending message to queue");
            }

            libc::waitpid(pid, std::ptr::null_mut(), 0);
        } 
        else if pid == 0 {
            libc::msgrcv(msgid, &mut buff as *mut _ as *mut libc::c_void, mem::size_of::<msgbuf>() - mem::size_of::<libc::c_long>(), 9527, 0);
            
            let received_text = std::str::from_utf8(CString::from_vec_unchecked(buff.mtext.to_vec()).as_bytes()).unwrap();
            let received_number = buff.number;

            println!("Child process read from msg: {}, {}", received_text, received_number);
        } 
        else {
            panic!("Fork failed");
        }
    }
}
extern crate libc;

use libc::{c_long, c_char, c_int, key_t, msgget, IPC_PRIVATE, IPC_EXCL, msgbuf, msgsnd, msgrcv, msgctl, IPC_CREAT, SEM_UNDO, MSG_NOERROR};
use std::ffi::{CString};

const MY_TYPE: c_long = 9527;
const MSG_SIZE: usize = 100;

fn main() {
    let ftok_path = CString::new("/tmp").expect("Error converting string");
    let proj_id: i32 = 42; // A constant unique ID
 
    let key = unsafe { libc::ftok(ftok_path.as_ptr(), proj_id) };
    if key == -1 {
        panic!("ftok failed in Rust!");
    }
    
    let msgid = unsafe { msgget(key, IPC_CREAT | 0o666) };
    if msgid == -1 {
        panic!("msgget failed in Rust!");
    }
 
    let pid = unsafe { libc::fork() };
 
    if pid > 0 {
        println!("Parent process started");
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        let mut buf: msgbuf = msgbuf { mtype: MY_TYPE, mtext: [0; MSG_SIZE], number: 0 };
        let message = CString::new("Hello from parent!").expect("Error converting string");
        let msg = message.as_bytes_with_nul();
        buf.mtext[..msg.len()].copy_from_slice(msg);
        buf.number = 123;
 
        unsafe {
            msgsnd(msgid, &buf, std::mem::size_of::<msgbuf>() as c_long, 0);
        }
        
        unsafe {
            libc::waitpid(pid, std::ptr::null_mut(), 0);
            msgctl(msgid, IPC_EXCL, std::ptr::null());
        }
    } else if pid == 0 {
        println!("Child process started");
        
        let mut buf: msgbuf = msgbuf { mtype: MY_TYPE, mtext: [0; MSG_SIZE], number: 0 };
 
        unsafe {
            msgrcv(msgid, &mut buf, std::mem::size_of::<msgbuf>() as c_long - std::mem::size_of_val(&buf.mtype) as i64, MY_TYPE, MSG_NOERROR);
        }
        
        println!("Child process read from msg: {:?}", buf.mtext.iter().map(|&c| c as char).collect::<String>());
        println!("Child process read from msg: {}", buf.number);
    } else {
        panic!("fork failed in Rust!");
    }
}
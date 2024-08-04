use std::mem;
use libc::{msgget, fork, msgctl, waitpid, msgrcv, msgsnd, IPC_CREAT, MSG_EXCEPT, MSG_NOERROR, IPC_RMID, c_void, getpid};
use std::ffi::CString;
use std::io;
use std::ptr;

extern crate libc;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    unsafe {
        let msgid = msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL);

        if msgid == -1 {
            perror("msgget");
            return;
        }

        let pid = fork();

        if pid > 0 {
            let mut buff: MsgBuf = mem::zeroed();
            buff.mtype = 9527;

            println!("Please enter a string you want to send:");
            let mut input_str = String::new();
            io::stdin().read_line(&mut input_str).expect("Failed to read line");
            input_str = input_str.trim().to_string();
            
            let str_c = CString::new(input_str.clone()).unwrap();
            ptr::copy_nonoverlapping(str_c.as_ptr(), buff.mtext.as_mut_ptr(), 100);

            println!("Please enter a number you want to send:");
            let mut input_num = String::new();
            io::stdin().read_line(&mut input_num).expect("Failed to read line");
            let num = input_num.trim().parse::<i32>().unwrap();
            buff.number = num;

            msgsnd(msgid, &buff as *const MsgBuf as *const u8, mem::size_of::<MsgBuf>(), 0);

            waitpid(pid, ptr::null_mut(), 0);
        } else if pid == 0 {
            let mut buff: MsgBuf = mem::zeroed();
            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut u8, mem::size_of::<MsgBuf>(), 9527, MSG_NOERROR);
            let received_string = CString::from_raw(buff.mtext.as_mut_ptr() as *mut i8).to_str().unwrap();
            let received_num = buff.number;
            
            println!("Child process read from msg: {}, {}", received_string, received_num);
        }
    }
}

fn perror(msg: &str) {
    unsafe {
        let c_msg = CString::new(msg).unwrap();
        libc::perror(c_msg.as_ptr());
    }
}
use std::process::Command;
use libc::{key_t, msgget, msgbuf, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, msgget, msgsnd, msgrcv};
use std::ffi::CString;
use std::mem;
use std::ptr;

const MY_TYPE: i64 = 9527;

fn main() {
    let key = IPC_PRIVATE;
    let msgid = unsafe { msgget(key, IPC_CREAT | IPC_EXCL | 0o666) };

    if msgid == -1 {
        panic!("Error creating message queue");
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        unsafe {
            Command::new("sleep")
                .arg("1")
                .spawn()
                .expect("failed to execute process");

            let mut buff: msgbuf = mem::zeroed();
            buff.mtype = MY_TYPE;

            let msg_text = CString::new("Hello from parent process").unwrap();
            let msg_number = 42;

            ptr::copy_nonoverlapping(msg_text.as_ptr(), buff.mtext.as_mut_ptr() as *mut u8, msg_text.as_bytes().len());
            *(buff.mtext.as_mut_ptr().add(msg_text.as_bytes().len())) = b'\0';

            msgsnd(msgid, &buff as *const msgbuf as *const i8, mem::size_of::<msgbuf>() - 8, 0);

            libc::waitpid(pid, ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        unsafe {
            let mut buff: msgbuf = mem::zeroed();
            buff.mtype = MY_TYPE;

            msgrcv(msgid, &mut buff as *mut msgbuf as *mut i8, mem::size_of::<msgbuf>() - 8, MY_TYPE, 0);
            
            let received_text = CString::from_vec_unchecked(buff.mtext.iter().cloned().take_while(|&c| c != 0).collect());
            let received_number = buff.number;

            println!("Child process read from msg: {:?}, {:?}", received_text, received_number);
        }
    } else {
        panic!("Error forking process");
    }
}
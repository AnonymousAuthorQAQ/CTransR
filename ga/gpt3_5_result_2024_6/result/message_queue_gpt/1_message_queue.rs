use libc::{c_long, c_int, size_t, fork, msgget, msgctl, msgrcv, msgsnd, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, MSG_NOERROR};
use std::ptr;
use std::mem;
use std::ffi::{CString, CStr};

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: c_int,
}

fn main() {
    let mut msgid = unsafe { msgget(IPC_PRIVATE, IPC_CREAT | IPC_EXCL | 0o666) };

    if msgid == -1 {
        panic!("Error creating message queue");
    }

    unsafe {
        let pid = fork();
        let mut buff: MsgBuf = mem::zeroed();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            buff.mtype = 9527;
            println!("Please enter a string you want to send:");
            let mut input_str = String::new();
            std::io::stdin().read_line(&mut input_str).expect("Failed to read input");
            let c_input_str = CString::new(input_str.trim()).expect("CString conversion failed");
            ptr::copy_nonoverlapping(c_input_str.as_ptr(), buff.mtext.as_mut_ptr(), c_input_str.as_bytes().len());
            
            println!("Please enter a number you want to send:");
            let mut input_num = String::new();
            std::io::stdin().read_line(&mut input_num).expect("Failed to read input");
            let input_num: c_int = input_num.trim().parse().expect("Input is not a valid integer");
            buff.number = input_num;

            let ret = msgsnd(msgid, &mut buff as *mut MsgBuf as *const _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>() as size_t, 0);
            if ret == -1 {
                panic!("Error sending message");
            }
        } else if pid == 0 {
            println!("Child process is waiting for msg:");
            let ret = msgrcv(msgid, &mut buff as *mut MsgBuf as *mut _, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>() as size_t, 9527, MSG_NOERROR);
            if ret == -1 {
                panic!("Error receiving message");
            }
            let received_str = CStr::from_ptr(buff.mtext.as_ptr() as *const _).to_str().expect("Invalid string received");
            println!("Child process read from msg: {}, {}", received_str, buff.number);
        } else {
            panic!("Fork failed");
        }
        
        if pid > 0 {
            let mut status: c_int = 0;
            unsafe { libc::waitpid(pid, &mut status as *mut c_int, 0); }
        }
        msgctl(msgid, libc::IPC_RMID, 0 as *mut _);
    }
}  
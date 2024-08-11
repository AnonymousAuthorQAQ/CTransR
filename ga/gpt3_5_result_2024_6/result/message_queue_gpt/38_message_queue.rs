use std::os::unix::io::AsRawFd;
use std::mem;
use std::ffi::{CString, CStr};
use std::ptr;
use libc::{c_long, c_int, msgrcv, msgsnd, msgget, IPC_CREAT, IPC_EXCL, ftok, fork, wait, waitpid, getpid};
use std::process;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [libc::c_char; 100],
    number: c_int,
}

fn main() {
    let key = ftok(CString::new(".").unwrap().as_ptr(), 65);
    if key == -1 {
        eprintln!("Error creating key");
        process::exit(1);
    }

    let msgid = unsafe { msgget(key, IPC_CREAT | IPC_EXCL | 0o666) };
    if msgid == -1 {
        eprintln!("Error creating message queue");
        process::exit(1);
    }

    unsafe {
        let pid = fork();
    
        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
    
            let mut buff: MsgBuf = mem::zeroed();
            buff.mtype = 9527;
    
            println!("Please enter a string you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().as_bytes();
            buff.mtext[..input.len()].copy_from_slice(input);
    
            println!("Please enter a number you want to send:");
            let mut number = String::new();
            std::io::stdin().read_line(&mut number).unwrap();
            let number: i32 = number.trim().parse().unwrap();
            buff.number = number;
    
            msgsnd(msgid, &buff as *const MsgBuf as *const libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), 0);
    
            waitpid(pid, ptr::null_mut(), 0);
        } else if pid == 0 {
            let mut buff: MsgBuf = mem::zeroed();
    
            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), 9527, 0);
            let received_string = CStr::from_ptr(buff.mtext.as_ptr()).to_str().unwrap();
            println!("Child process read from msg: {}, {}", received_string, buff.number);
        } else {
            eprintln!("Error forking process");
            process::exit(1);
        }
    }
}
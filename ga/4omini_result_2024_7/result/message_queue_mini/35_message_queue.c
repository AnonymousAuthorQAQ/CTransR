use libc::{c_int, c_void, pid_t, IPC_PRIVATE, MSG_NOERROR};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [i8; 100],
    number: c_int,
}

extern "C" {
    fn msgget(key: c_int, msgflg: c_int) -> c_int;
    fn msgsnd(msqid: c_int, msgp: *const MsgBuf, msgsz: usize, msgflg: c_int) -> c_int;
    fn msgrcv(msqid: c_int, msgp: *mut MsgBuf, msgsz: usize, msgtyp: c_long, msgflg: c_int) -> c_int;
}

fn main() {
    unsafe {
        let msgid = msgget(IPC_PRIVATE, 0o666 | 0o2000); // IPC_EXCL is 0o2000

        if msgid == -1 {
            eprintln!("msgget failed");
            exit(1);
        }

        let pid = libc::fork();

        if pid > 0 {
            libc::sleep(1);

            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Input string
            println!("Please enter a string you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let c_str = CString::new(input.trim()).expect("CString::new failed");
            for (i, &byte) in c_str.as_bytes().iter().enumerate() {
                buff.mtext[i] = byte as i8;
            }

            // Input number
            println!("Please enter a number you want to send:");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            buff.number = input.trim().parse::<c_int>().expect("Failed to parse number");

            msgsnd(msgid, &buff, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), 0);

            // Wait for child process to finish
            libc::waitpid(pid, ptr::null_mut(), 0);
        } else if pid == 0 {
            let mut buff: MsgBuf = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            msgrcv(msgid, &mut buff, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), MY_TYPE, 0);
            let received_string = CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy().into_owned();
            println!("Child process read from msg: {}, {}", received_string, buff.number);
        } else {
            eprintln!("fork failed");
            exit(1);
        }
    }
}
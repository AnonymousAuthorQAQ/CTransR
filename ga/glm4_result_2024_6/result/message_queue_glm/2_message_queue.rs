use std::io;
use std::process::Command;
use std::ptr;
use std::str;
use std::ffi::CString;
use std::os::raw::c_char;

const MY_TYPE: i32 = 9527;

fn main() {
    let msgid = unsafe { msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_CREAT | libc::IPC_EXCL) };
    
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    match unsafe { fork() } {
        0 => {
            let mut buff: libc::msgbuf = libc::msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            if msgrcv(msgid, &mut buff, 100, MY_TYPE, 0) == -1 {
                eprintln!("msgrcv failed");
                return;
            }

            println!("Child process read from msg: {}", str::from_utf8(&buff.mtext).unwrap());
            println!("Child process read from msg: {}", buff.number);
        },
        pid => {
            unsafe {
                sleep(1);
            }

            let mut buff: libc::msgbuf = libc::msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            println!("Please enter a string you want to send:");
            io::stdin().read_line(&mut buff.mtext).unwrap();
            println!("Please enter a number you want to send:");
            io::stdin().read_line(&mut buff.mtext).unwrap();
            buff.number = buff.mtext.trim().parse().unwrap();

            if msgsnd(msgid, &buff, 100, 0) == -1 {
                eprintln!("msgsnd failed");
                return;
            }

            waitpid(pid, ptr::null_mut(), 0);
        },
    }
}

extern "C" {
    fn msgget(key: i32, msgflg: i32) -> i32;
    fn msgsnd(msgid: i32, msgp: *const libc::msgbuf, msgsz: usize, msgflg: i32) -> i32;
    fn msgrcv(msgid: i32, msgp: *mut libc::msgbuf, msgsz: usize, msgtyp: i32, msgflg: i32) -> i32;
    fn fork() -> i32;
    fn waitpid(pid: i32, status: *mut i32, options: i32) -> i32;
    fn sleep(seconds: u32);
}
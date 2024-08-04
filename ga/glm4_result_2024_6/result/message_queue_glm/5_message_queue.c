use std::io::{self, Read};
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::ptr;
use std::ffi::CString;
use std::mem;
use std::time::Duration;
use std::thread::sleep;

const MY_TYPE: i32 = 9527;

fn main() {
    let msgid = unsafe { msgget(libc::IPC_PRIVATE, 0666 | libc::IPC_CREAT) };
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    let pid = unsafe { fork() };
    if pid > 0 {
        sleep(Duration::from_secs(1));

        let mut buff = libc::msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        print!("Please enter a string you want to send:\n");
        io::stdin().read_line(&mut buff.mtext).unwrap();
        print!("Please enter a number you want to send:\n");
        io::stdin().read_line(&mut buff.mtext).unwrap();
        buff.number = buff.mtext.trim().parse().unwrap();

        unsafe {
            msgsnd(msgid, &buff, mem::size_of_val(&buff) - mem::size_of_val(&buff.mtype), 0);
        }

        unsafe {
            waitpid(pid, ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff = libc::msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        println!("Child process is waiting for msg:");
        unsafe {
            msgrcv(msgid, &mut buff, mem::size_of_val(&buff) - mem::size_of_val(&buff.mtype), MY_TYPE, 0);
        }
        println!("Child process read from msg: {}", buff.mtext, buff.number);
    } else {
        eprintln!("fork failed");
    }

    unsafe {
        msgctl(msgid, libc::IPC_RMID, ptr::null_mut());
    }
}

#[link(name = "c")]
extern "C" {
    fn fork() -> i32;
    fn waitpid(pid: i32, status: *mut i32, options: i32) -> i32;
    fn msgget(key: i32, msgflg: i32) -> i32;
    fn msgsnd(msgid: i32, msgp: *const libc::msgbuf, msgsz: usize, msgflg: i32) -> i32;
    fn msgrcv(msgid: i32, msgp: *mut libc::msgbuf, msgsz: usize, msgtyp: i32, msgflg: i32) -> i32;
    fn msgctl(msgid: i32, cmd: i32, buf: *mut libc::msqid_ds) -> i32;
}
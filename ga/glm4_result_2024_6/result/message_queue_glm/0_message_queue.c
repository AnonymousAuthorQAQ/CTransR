use std::io::{self, Read};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::ptr::null_mut;
use std::ffi::CString;
use std::env;

fn main() {
    let msgid = unsafe { msgget(libc::IPC_PRIVATE, 0666 | libc::IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Failed to create message queue");
        return;
    }

    match unsafe { fork() } {
        Ok(0) => {
            let mut buff = libc::msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            if unsafe { msgrcv(msgid, &mut buff, 100, MY_TYPE, 0) } == -1 {
                eprintln!("Failed to receive message");
                return;
            }

            println!("Child process read from msg: {}", unsafe {
                CString::from_raw(buff.mtext.as_mut_ptr())
            });
        }
        Ok(pid) => {
            unsafe {
                sleep(1);
            }

            println!("Please enter a string you want to send:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let text = CString::new(input.trim()).unwrap();
            println!("Please enter a number you want to send:");
            let number: i32 = io::stdin().read_line().unwrap().trim().parse().unwrap();

            let mut buff = libc::msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number,
            };

            unsafe {
                strcpy(buff.mtext.as_mut_ptr(), text.as_ptr());
                msgsnd(msgid, &buff, 100, 0);
            }

            waitpid(pid, null_mut(), 0);
        }
        Err(e) => {
            eprintln!("Failed to fork process: {}", e);
        }
    }
}

extern "C" {
    fn msgget(key: libc::key_t, flags: libc::c_int) -> libc::c_int;
    fn msgsnd(msgid: libc::c_int, msgp: *const libc::msgbuf, msgsz: libc::c_long, flags: libc::c_int) -> libc::c_int;
    fn msgrcv(msgid: libc::c_int, msgp: *mut libc::msgbuf, msgsz: libc::c_long, msgtyp: libc::c_long, flags: libc::c_int) -> libc::c_int;
    fn fork() -> libc::c_int;
    fn waitpid(pid: libc::pid_t, status: *mut libc::c_int, options: libc::c_int) -> libc::pid_t;
    fn sleep(seconds: libc::c_uint);
    fn strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
    fn perror(s: *const libc::c_char);
}

const MY_TYPE: libc::c_long = 9527;
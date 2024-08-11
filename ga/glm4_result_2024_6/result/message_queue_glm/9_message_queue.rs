use std::ffi::CString;
use std::io::{self, Read};
use std::os::raw::c_long;
use std::ptr;
use std::thread;
use std::time::Duration;
use std::wait::WaitStatus;

use libc::{fork, gets, msgget, msgrcv, msgsnd, IPC_PRIVATE, IPC_EXCL, IPC_CREAT, IPC_NOWAIT, waitpid};

const MY_TYPE: c_long = 9527;

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, IPC_CREAT | IPC_EXCL | 0o666) };

    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    match unsafe { fork() } {
        0 => {
            // Child process
            let mut buff = libc::msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            let _ = thread::sleep(Duration::from_secs(1));

            let _ = io::stdin().read_line(&mut buff.mtext);
            let _ = io::stdin().read_line(&mut buff.number);

            if unsafe { msgsnd(msgid, &buff, (std::mem::size_of::<libc::msgbuf>() - std::mem::size_of::<c_long>()) as i32, IPC_NOWAIT) } == -1 {
                eprintln!("msgsnd failed");
            }
        }
        pid if pid > 0 => {
            // Parent process
            let mut buff = libc::msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            if unsafe { msgrcv(msgid, &mut buff, (std::mem::size_of::<libc::msgbuf>() - std::mem::size_of::<c_long>()) as i32, MY_TYPE, 0) } == -1 {
                eprintln!("msgrcv failed");
            } else {
                println!("Parent process read from msg: {}", buff.mtext);
            }

            let _ = unsafe { waitpid(pid, ptr::null_mut(), 0) };
        }
        _ => {
            eprintln!("fork failed");
        }
    }
}
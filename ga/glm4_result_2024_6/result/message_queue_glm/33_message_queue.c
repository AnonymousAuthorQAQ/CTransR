use std::io::{self, Read};
use std::process::wait;
use std::ptr::null;
use std::ffi::CString;
use std::os::raw::{c_int, c_long};
use std::ptr;

use libc::{fork, getpid, sleep, msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL, IPC_CREAT, IPC_NOWAIT, msgbuf};

fn main() {
    unsafe {
        let msgid = msgget(IPC_PRIVATE, IPC_CREAT | IPC_EXCL | 0o666);
        if msgid == -1 {
            eprintln!("msgget failed");
            return;
        }

        let pid = fork();
        if pid > 0 {
            sleep(1);

            let mut buff = msgbuf {
                mtype: 9527,
                mtext: [0; 100],
                number: 0,
            };

            println!("Please enter a string to send: ");
            io::stdin().read_line(&mut buff.mtext).unwrap();

            println!("Please enter a number to send: ");
            io::stdin().read_line(&mut buff.number).unwrap();

            if msgsnd(msgid, &buff, 104, IPC_NOWAIT) == -1 {
                eprintln!("msgsnd failed");
            }

            wait();
        } else if pid == 0 {
            let mut buff = msgbuf {
                mtype: 9527,
                mtext: [0; 100],
                number: 0,
            };

            if msgrcv(msgid, &mut buff, 104, 9527, IPC_NOWAIT) == -1 {
                eprintln!("msgrcv failed");
            }

            println!("Child process read from msg: {}", buff.mtext);
        } else {
            eprintln!("fork failed");
        }
    }
}
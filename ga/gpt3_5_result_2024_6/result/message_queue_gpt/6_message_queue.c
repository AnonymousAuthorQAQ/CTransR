use std::process;
use std::ptr;
use libc::{key_t, ftok, shmget, shmat, IPC_PRIVATE, IPC_EXCL, msgget, msgsnd, msgrcv};

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let msgid: i32;
    let mut pid: i32;

    unsafe {
        msgid = msgget(IPC_PRIVATE, 0o666 | IPC_EXCL);

        if msgid == -1 {
            perror("msgget");
            return;
        }

        pid = fork();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff = MsgBuf {
                mtype: 9527,
                mtext: [0; 100],
                number: 0,
            };

            // println!("Please enter a string you want to send:");
            std::io::stdin().read_line(&mut buff.mtext).unwrap();
            // println!("Please enter a number you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            buff.number = input.trim().parse().unwrap();

            msgsnd(msgid, &buff as *const MsgBuf as *const std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0);

            let _ = process::wait();
        } else if pid == 0 {
            // println!("Child process is waiting for msg:");
            let mut buff = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 9527, 0);
            println!("Child process read from msg: {}, {}", std::str::from_utf8(&buff.mtext).unwrap(), buff.number);
        } else {
            perror("fork");
        }
    }
}

unsafe fn perror(msg: &str) {
    let err = std::io::Error::last_os_error();
    eprintln!("{}: {}", msg, err);
}
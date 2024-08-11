use std::mem;
use std::os::unix::io::RawFd;
use libc::{msgget, msgctl, msgsnd, msgrcv, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, msgbuf, msqid_ds, msgctl_cmd, MSG_NOERROR};
use std::ffi::CString;
use std::ptr;
use std::process;

const MY_TYPE: i64 = 9527;

fn main() {
    let mut msqid: i32;
    let mut pid: i32;

    let mut buf: msgbuf = msgbuf {
        mtype: 0,
        mtext: [0; 100],
    };

    unsafe {
        msqid = msgget(IPC_PRIVATE, IPC_CREAT | IPC_EXCL | 0o666);
        if msqid == -1 {
            perror("msgget");
            process::exit(-1);
        }

        pid = libc::fork();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            buf.mtype = MY_TYPE;

            let mut input = String::new();
            println!("Please enter a string you want to send:");
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            libc::strcpy(buf.mtext.as_mut_ptr(), input.as_ptr() as *const i8);

            println!("Please enter a number you want to send:");
            let mut number = String::new();
            std::io::stdin().read_line(&mut number).unwrap();
            let number: i32 = number.trim().parse().unwrap();
            buf.number = number;

            let ptr = &buf as *const _ as *const i8;
            msgsnd(msqid, ptr, mem::size_of::<msgbuf>() - mem::size_of::<libc::msgtype_t>(), 0);

            libc::waitpid(pid, ptr::null_mut(), 0);
        } else if pid == 0 {
            println!("Child process is waiting for msg:");
            let ptr = &mut buf as *mut _ as *mut i8;
            msgrcv(msqid, ptr, mem::size_of::<msgbuf>() - mem::size_of::<libc::msgtype_t>(), MY_TYPE, MSG_NOERROR);
            let text = std::ffi::CStr::from_ptr(buf.mtext.as_ptr()).to_str().unwrap();
            println!("Child process read from msg: {}, {}", text, buf.number);
        } else {
            perror("fork");
        }
    }
}

fn perror(msg: &str) {
    unsafe {
        let c_msg = CString::new(msg).unwrap();
        libc::perror(c_msg.as_ptr());
    }
}
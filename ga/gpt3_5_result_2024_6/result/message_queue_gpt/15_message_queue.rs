use std::process;
use std::ptr;
use libc::{msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL, fork, waitpid, gets, scanf};

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error: msgget failed");
        process::exit(-1);
    }

    let pid = unsafe { fork() };

    if pid > 0 {
        unsafe { libc::sleep(1) };

        let mut buff = libc::msgbuf {
            mtype: 9527,
            mtext: [0; 100],
            number: 0,
        };

        println!("Please enter a string you want to send:");
        unsafe { gets(buff.mtext.as_mut_ptr()) };

        println!("Please enter a nubmer you want to send:");
        unsafe { scanf(b"%d\0".as_ptr(), &mut buff.number) };

        unsafe { msgsnd(msgid, &buff as *const libc::msgbuf as *const _, std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype) as isize, 0) };

        unsafe { waitpid(pid, std::ptr::null_mut(), 0) };
    } else if pid == 0 {
        let mut buff = libc::msgbuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        println!("Child process is waiting for msg:");
        unsafe { msgrcv(msgid, &mut buff as *mut libc::msgbuf as *mut _, std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype) as isize, 9527, 0) };

        println!("Child process read from msg: {}, {}", unsafe { std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_str().unwrap() }, buff.number);
    }
    unsafe { libc::_exit(0) };
}
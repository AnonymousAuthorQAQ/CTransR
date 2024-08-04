use std::os::unix::prelude::RawFd;
use std::process::{Command, exit};
use libc::{msgget, msgrcv, msgsnd, fork, waitpid, c_void, c_long, getpid, message_queue::msgbuf, IPC_CREAT, IPC_EXCL, MSG_SIZE};

const MY_TYPE: i64 = 9527;
const MSG_SIZE: usize = 100;

fn main() {
    unsafe {
        let mut msgid = msgget(0, IPC_CREAT | IPC_EXCL | 0o666); 
        
        if msgid == -1 {
            perror("msgget");
            exit(-1);
        }

        let pid = fork();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::new(1, 0));

            let mut buff = msgbuf {
                mtype: MY_TYPE,
                mtext: [0; MSG_SIZE],
                number: 0,
            };

            println!("Please enter a string you want to send:");
            let mut input_string = String::new();
            std::io::stdin().read_line(&mut input_string).unwrap();
            let input_string = input_string.trim().to_string();
            std::ptr::copy_nonoverlapping(input_string.as_ptr(), buff.mtext.as_mut_ptr(), input_string.len());
            buff.mtext[input_string.len()] = 0;

            println!("Please enter a number you want to send:");
            let mut input_number = String::new();
            std::io::stdin().read_line(&mut input_number).unwrap();
            buff.number = input_number.trim().parse().unwrap();

            msgsnd(msgid, &buff as *const _ as *const c_void, std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype), 0);

            waitpid(pid, std::ptr::null_mut(), 0);
        } else if pid == 0 {
            let mut buff = msgbuf {
                mtype: 0,
                mtext: [0; MSG_SIZE],
                number: 0,
            };

            println!("Child process is waiting for msg:");
            msgrcv(msgid, &mut buff as *mut _ as *mut c_void, std::mem::size_of_val(&buff) - std::mem::size_of_val(&buff.mtype), MY_TYPE, 0);
            let received_string = std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_str().unwrap();
            println!("Child process read from msg: {}, {}", received_string, buff.number);
        } else {
            perror("fork");
        }
    }
}

fn perror(msg: &str) {
    eprintln!("{}: {}", msg, std::io::Error::last_os_error());
}
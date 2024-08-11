use std::process::Command;
use std::os::unix::process::CommandExt;
use std::mem;
use std::ptr;
use libc::{self, c_long, c_void, size_t};

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct Msgbuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: c_int
}

fn main() {
    let mut msgid = unsafe {
        libc::msgget(libc::IPC_PRIVATE, 0o666)
    };

    if msgid == -1 {
        panic!("Failed to get message queue");
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff: Msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0
        };

        println!("Please enter a string you want to send:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop(); // Remove newline character
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr() as *const c_void, buff.mtext.as_mut_ptr() as *mut c_void, input.len());
        }

        println!("Please enter a number you want to send:");
        let mut number_input = String::new();
        std::io::stdin().read_line(&mut number_input).unwrap();
        let number: i32 = number_input.trim().parse().unwrap();

        buff.number = number;

        unsafe {
            libc::msgsnd(msgid, &buff as *const Msgbuf as *const c_void, mem::size_of::<Msgbuf>() - mem::size_of::<c_long>(), 0);
        }

        let mut status: c_int = 0;
        unsafe {
            libc::waitpid(pid, &mut status, 0);
        }
    } else if pid == 0 {
        println!("Child process is waiting for msg:");
        let mut buff: Msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0
        };

        unsafe {
            libc::msgrcv(msgid, &mut buff as *mut Msgbuf as *mut c_void, mem::size_of::<Msgbuf>() - mem::size_of::<c_long>(), MY_TYPE, 0);
        }

        let message = std::str::from_utf8(CStr::from_ptr(buff.mtext.as_ptr()).to_bytes()).unwrap();
        println!("Child process read from msg: {}, {}", message, buff.number);
    } else {
        panic!("Fork failed");
    }
}
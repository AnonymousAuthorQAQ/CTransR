use std::ptr;
use std::os::unix::prelude::RawFd;
use std::ffi::CString;
use std::mem;
use libc::{msgget, fork, waitpid, c_int, c_void, msgrcv, msgsnd, IPC_CREAT, IPC_EXCL, MSG_NOERROR, S_IWUSR, S_IRUSR, getppid, c_long, getpid, c_char};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [c_char; 100],
    number: i32,
}

fn main() {
    let msgid = unsafe { msgget(IPC_CREAT | IPC_EXCL | S_IWUSR | S_IRUSR, S_IWUSR | S_IRUSR) };
    if msgid == -1 {
        panic!("Failed to create message queue");
    }

    unsafe {
        let pid = fork();
        if pid < 0 {
            panic!("Fork failed");
        } else if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff: MsgBuf = mem::zeroed();
            buff.mtype = MY_TYPE;

            let string = get_input("Please enter a string you want to send:");
            let number = get_input("Please enter a number you want to send:").parse::<i32>().unwrap();

            ptr::copy_nonoverlapping(string.as_ptr() as *const c_char, buff.mtext.as_mut_ptr(), string.len());
            buff.number = number;

            let ret = msgsnd(msgid, &buff as *const MsgBuf as *const c_void, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), MSG_NOERROR);
            if ret == -1 {
                panic!("Failed to send message");
            }

            waitpid(pid, std::ptr::null_mut(), 0);
        } else {
            let mut buff: MsgBuf = mem::zeroed();
            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut c_void, mem::size_of::<MsgBuf>() - mem::size_of::<c_long>(), MY_TYPE, 0);
            let received_string = CString::from_raw(buff.mtext.as_mut_ptr()).to_str().unwrap();
            println!("Child process read from msg: {}, {}", received_string, buff.number);
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
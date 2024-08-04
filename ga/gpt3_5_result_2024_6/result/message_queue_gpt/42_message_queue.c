use std::ptr;
use std::ffi::CString;
use libc::{c_long, c_void, size_t, msgbuf, msgget, msgsnd, msgrcv, ftok, fork, getpid, waitpid, perror};
use std::io::{self, Write};

const IPC_CREATE: i32 = 0666;
const IPC_PRIVATE: i32 = 0;

const KEY: i32 = 1234;
const MY_TYPE: i64 = 9527;

fn main() {
    let msgid = unsafe { msgget(ftok(CString::new("/tmp").unwrap().as_ptr(), KEY) as i32, IPC_CREATE | IPC_PRIVATE) };

    if msgid == -1 {
        unsafe {
            perror("msgget".as_ptr());
        }
        return;
    }

    let pid = unsafe { fork() };

    if pid > 0 {
        unsafe {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        let mut buff: msgbuf = msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        print!("Please enter a string you want to send:\n");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let input_cstring = CString::new(input).unwrap();
        unsafe {
            ptr::copy_nonoverlapping(input_cstring.as_ptr(), buff.mtext.as_mut_ptr() as *mut u8, input_cstring.as_bytes().len());
        }

        print!("Please enter a number you want to send:\n");
        io::stdout().flush().unwrap();
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input).unwrap();
        let number_input: i32 = number_input.trim().parse().unwrap();
        buff.number = number_input;

        unsafe {
            msgsnd(msgid, &buff as *const msgbuf as *const c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), 0);
            waitpid(pid, std::ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff: msgbuf = msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        unsafe {
            msgrcv(msgid, &mut buff as *mut msgbuf as *mut c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<c_long>(), MY_TYPE as i64, 0);
            println!("Child process read from msg: {}, {}", CString::from_raw(buff.mtext.as_mut_ptr()).to_str().unwrap(), buff.number);
        }
    } else {
        unsafe {
            perror("fork\0".as_ptr() as *const i8);
        }
    }
}
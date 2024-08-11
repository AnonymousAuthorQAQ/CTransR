use libc::{msgsnd, msgrcv, msgget, key_t, ftok, IPC_CREAT, IPC_PRIVATE, 0666, IPC_EXCL,  msgbuf};
use std::mem::size_of;
use std::ptr;
use std::ffi::CString;
use std::io::{stdin, stdout, Write};

const MY_TYPE: i64 = 9527;

fn main() {
    let key = ftok(CString::new(".").expect("ftok failed").as_ptr() as _, 1);

    let msgid = unsafe { msgget(key, IPC_CREAT | IPC_PRIVATE | 0666 | IPC_EXCL) };

    if msgid == -1 {
        panic!("msgget failed");
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        unsafe {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff: msgbuf = std::mem::zeroed();
            buff.mtype = MY_TYPE;

            print!("Please enter a string you want to send: ");
            let _ = stdout().flush();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.pop(); // Removing '\n'
            
            let input_c = CString::new(input.clone()).expect("Failed to convert input to CString");
            ptr::copy_nonoverlapping(input_c.as_ptr(), buff.mtext.as_mut_ptr(), input.len() + 1);
            
            println!("Please enter a number you want to send: ");
            let _ = stdout().flush();
            let mut number = String::new();
            stdin().read_line(&mut number).unwrap();
            let number: i32 = number.trim().parse().expect("Invalid number");
            
            buff.number = number;

            msgsnd(msgid, &buff as *const _ as *const libc::c_void, size_of::<msgbuf>() - size_of::<libc::c_long>(), 0);

            libc::waitpid(pid, std::ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff: msgbuf = std::mem::zeroed();

        unsafe {
            stdout().flush().unwrap();
            msgrcv(msgid, &mut buff as *mut _ as *mut libc::c_void, size_of::<msgbuf>() - size_of::<libc::c_long>(), MY_TYPE, 0);
            let message = std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_string_lossy();
            println!("Child process read from msg: {}, {}", message, buff.number);
        }
    } else {
        panic!("fork failed");
    }
}
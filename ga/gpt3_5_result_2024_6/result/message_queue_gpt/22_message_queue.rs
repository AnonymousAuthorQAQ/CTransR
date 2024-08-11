use std::ptr;
use std::ffi::CString;
use std::os::raw::c_char;
use libc::{msgget, msgsnd, msgrcv, fork, getpid, waitpid, IPC_PRIVATE, IPC_EXCL};

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error in msgget");
        return;
    }

    let pid = unsafe { fork() };

    match pid {
        -1 => eprintln!("Error in fork"),
        0 => {
            let mut buff: MsgBuf = unsafe { std::mem::zeroed() };

            unsafe {
                msgrcv(msgid, &mut buff as *mut _ as *mut std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), MY_TYPE, 0);
                let msg_text = CString::from_raw(buff.mtext.as_mut_ptr() as *mut c_char).into_string().unwrap();
                println!("Child process read from msg: {}, {}", msg_text, buff.number);
            }
        }
        _ => {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let mut buff: MsgBuf = unsafe { std::mem::zeroed() };

            unsafe {
                buff.mtype = MY_TYPE;
                print!("Please enter a string you want to send: ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_string();
                let c_input = CString::new(input).unwrap();
                ptr::copy_nonoverlapping(c_input.as_ptr(), buff.mtext.as_mut_ptr() as *mut u8, input.len());
                
                print!("Please enter a number you want to send: ");
                let mut input_number = String::new();
                std::io::stdin().read_line(&mut input_number).unwrap();
                let number = input_number.trim().parse().unwrap();
                buff.number = number;

                msgsnd(msgid, &buff as *const _ as *const std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0);

                waitpid(pid, std::ptr::null_mut(), 0);
            }
        }
    }
}
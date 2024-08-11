use std::mem;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use libc::{msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL};
use std::ptr;
use std::process::Command;

const MY_TYPE: i64 = 9527;

struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let msgid: c_int;
    let mut pid: i32;

    unsafe {
        msgid = msgget(IPC_PRIVATE, 0o666 | IPC_EXCL);
        if msgid == -1 {
            println!("Error creating message queue");
            return;
        }

        pid = Command::new("sh")
            .arg("-c")
            .arg("echo $$")
            .output()
            .expect("Failed to execute command")
            .stdout
            .parse()
            .expect("Failed to parse output");

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            println!("Please enter a string you want to send:");
            let mut input_text = String::new();
            std::io::stdin().read_line(&mut input_text).expect("Failed to read line");
            let c_input_text = CString::new(input_text.trim()).expect("Failed to convert string to CString");
            let c_input_text_ptr = c_input_text.as_ptr() as *mut c_char;
            ptr::copy_nonoverlapping(c_input_text_ptr, buff.mtext.as_mut_ptr(), input_text.len());

            println!("Please enter a number you want to send:");
            let mut input_number = String::new();
            std::io::stdin().read_line(&mut input_number).expect("Failed to read line");
            buff.number = input_number.trim().parse().expect("Failed to parse number");

            msgsnd(msgid, &buff as *const _ as *const std::ffi::c_void, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), 0);

            waitpid(pid, std::ptr::null_mut(), 0);
        } else if pid == 0 {
            let mut buff = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };
            
            println!("Child process is waiting for message:");
            msgrcv(msgid, &mut buff as *mut _ as *mut std::ffi::c_void, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), MY_TYPE, 0);
            let received_text = std::str::from_utf8(&buff.mtext).expect("Failed to convert text to UTF-8");
            println!("Child process read from message: {}, {}", received_text, buff.number);
        } else {
            println!("Error forking process");
        }
    }
}
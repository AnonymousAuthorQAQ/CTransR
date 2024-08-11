use std::ffi::CString;
use std::os::raw::{c_long, c_void};
use std::mem;
use libc::{msgget, IPC_PRIVATE, MSG_EXCEPT, msgctl, msgsnd, msgrcv, fork, WAIT_ANY};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let msgid = unsafe { msgget(CString::new(IPC_PRIVATE.to_string()).unwrap().as_ptr(), 0o666) };
    if msgid == -1 {
        eprintln!("Error creating message queue");
        return;
    }

    let pid = unsafe { fork() };

    match pid {
        -1 => eprintln!("Fork failed."),
        0 => {
            let mut buff: MsgBuf = unsafe { mem::zeroed() };
            let msg_type = MY_TYPE;
            unsafe {
                msgrcv(msgid, &mut buff as *mut MsgBuf as *mut c_void, mem::size_of::<MsgBuf>() as i64, msg_type, 0);
                let received_string = String::from_utf8_lossy(&buff.mtext[..]);
                println!("Child process read from msg: {}, {}", received_string, buff.number);
            }
        }
        _ => {
            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            println!("Please enter a string you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            input.pop(); // remove newline character
            // Copy string to buffer, truncate or pad to fit 100 bytes
            let bytes = input.into_bytes();
            let len = bytes.len().min(100);
            buff.mtext[..len].clone_from_slice(&bytes[..len]);

            println!("Please enter a number you want to send:");
            let mut number_input = String::new();
            std::io::stdin().read_line(&mut number_input).expect("Failed to read line");
            let number: i32 = number_input.trim().parse().unwrap();
            buff.number = number;

            unsafe {
                msgsnd(msgid, &buff as *const MsgBuf as *const c_void, std::mem::size_of::<MsgBuf>() as i64 - std::mem::size_of::<c_long>(), 0);
            }

            let _ = unsafe { msgctl(msgid, IPC_PRIVATE, std::ptr::null_mut()) };

            let _ = unsafe { waitpid(-1, std::ptr::null_mut(), WAIT_ANY) };
        }
    }
}
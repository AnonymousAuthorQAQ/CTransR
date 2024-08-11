use std::process::Command;
use std::ptr;
use std::ffi::CString;
use libc::{msgget, msgsnd, msgrcv, IPC_CREAT, IPC_EXCL, key_t, c_long, c_int};
use std::mem::size_of;
use std::io::{Write, stdin};

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: c_int,
}

fn main() {
    let key = unsafe { msgget(0, IPC_CREAT | IPC_EXCL | 0o666) };
    if key == -1 {
        eprintln!("Error creating message queue");
        return;
    }

    let pid = unsafe { Command::new("sh").arg("-c").arg("echo $$").output().unwrap() };

    if pid.status.success() {
        let pid = String::from_utf8_lossy(&pid.stdout).trim();
        
        if let Ok(pid) = pid.parse::<i32>() {
            if pid > 0 {
                println!("Parent process with PID: {}", pid);
            }
        }
    }

    unsafe {
        let mut buff: MsgBuf = std::mem::zeroed();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            
            buff.mtype = MY_TYPE;
            print!("Please enter a string you want to send: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let c_input = CString::new(input).expect("CString::new failed");
            buff.mtext[..input.len()].copy_from_slice(c_input.as_bytes());
            
            print!("Please enter a number you want to send: ");
            std::io::stdout().flush().unwrap();
            let mut number = String::new();
            stdin().read_line(&mut number).unwrap();
            let number = number.trim().parse().unwrap();
            buff.number = number;

            msgsnd(key, &buff as *const MsgBuf as *mut _, size_of::<MsgBuf>() - size_of::<c_long>(), 0);
        } else if pid == 0 {
            println!("Child process is waiting for msg:");
            msgrcv(key, &mut buff as *mut MsgBuf as *mut _, size_of::<MsgBuf>() - size_of::<c_long>(), MY_TYPE, 0);
            let msg_text = std::str::from_utf8(&buff.mtext).unwrap();
            println!("Child process read from msg: {}, {}", msg_text, buff.number);
        } else {
            eprintln!("fork failed");
        }
    }
}

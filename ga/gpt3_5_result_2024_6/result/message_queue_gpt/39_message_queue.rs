use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::process::Command;

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [c_char; 100],
    number: i32,
}

fn main() {
    let mut msgid: i32;
    let mut pid: i32;

    unsafe {
        let key = libc::IPC_PRIVATE;
        msgid = libc::msgget(key, 0o666 | libc::IPC_EXCL);

        if msgid == -1 {
            perror("msgget");
            return;
        }
    }

    pid = unsafe { libc::fork() };

    if pid > 0 {
        unsafe {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff: MsgBuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            println!("Please enter a string you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let c_input = CString::new(input.trim()).unwrap();
            ptr::copy_nonoverlapping(c_input.as_ptr(), buff.mtext.as_mut_ptr(), c_input.as_bytes().len());

            println!("Please enter a number you want to send:");
            let mut number_input = String::new();
            std::io::stdin().read_line(&mut number_input).unwrap();
            buff.number = number_input.trim().parse().unwrap();

            libc::msgsnd(msgid, &buff as *const MsgBuf as *mut _, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), 0);

            std::thread::sleep(std::time::Duration::from_secs(1));
            Command::new("kill").arg("-9").arg(pid.to_string()).output().expect("Failed to kill child process");
        }
    } else if pid == 0 {
        unsafe {
            let mut buff: MsgBuf = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            println!("Child process is waiting for msg:");
            libc::msgrcv(msgid, &buff as *const MsgBuf as *mut _, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), MY_TYPE, 0);
            let received_str = std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_str().expect("Failed to convert received string to &str");
            println!("Child process read from msg: {}, {}", received_str, buff.number);
        }
    } else {
        eprintln!("Fork failed");
    }
}

unsafe fn perror(msg: &str) {
    let err_msg = CString::new(msg).unwrap();
    libc::perror(err_msg.as_ptr());
}
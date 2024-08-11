use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ptr;
use libc::{msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL};

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [c_char; 100],
    number: c_int,
}

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        println!("Please enter a string you want to send:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.pop(); // Remove newline character
        let input_cstring = std::ffi::CString::new(input).unwrap();
        unsafe {
            ptr::copy_nonoverlapping(input_cstring.as_ptr(), buff.mtext.as_mut_ptr() as *mut i8, input.len());
        }

        println!("Please enter a number you want to send:");
        let mut num_input = String::new();
        std::io::stdin().read_line(&mut num_input).expect("Failed to read line");
        let num_input: i32 = num_input.trim().parse().expect("Please enter a valid number");
        buff.number = num_input;

        unsafe {
            msgsnd(msgid, &buff as *const MsgBuf as *const std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0);
        }

        libc::waitpid(pid, std::ptr::null_mut(), 0);
    } else if pid == 0 {
        let mut buff = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };
        println!("Child process is waiting for msg:");

        unsafe {
            msgrcv(msgid, &mut buff as *mut MsgBuf as *mut std::ffi::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), MY_TYPE, 0);
        }

        let received_string = std::ffi::CStr::from_ptr(buff.mtext.as_ptr()).to_str().unwrap();
        println!("Child process read from msg: {}, {}", received_string, buff.number);
    } else {
        eprintln!("fork failed");
    }
}
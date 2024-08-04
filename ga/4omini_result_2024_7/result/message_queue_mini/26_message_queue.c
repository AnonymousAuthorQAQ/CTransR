use std::ffi::CString;
use std::mem;
use std::os::unix::io::RawFd;
use std::ptr;
use std::process::{Child, Command};
use std::io::{self, Write};
use std::os::unix::process::CommandExt;

const MY_TYPE: i64 = 9527;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [i8; 100],
    number: i32,
}

fn main() -> io::Result<()> {
    // Create a new message queue
    let msgid = unsafe { libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL) };
    if msgid == -1 {
        eprintln!("msgget failed");
        std::process::exit(1);
    }

    // Fork the process
    let pid = unsafe { libc::fork() };
    
    if pid > 0 {
        // Parent process
        libc::sleep(1);
        
        let mut buff: MsgBuf = unsafe { mem::zeroed() };
        buff.mtype = MY_TYPE;

        // Taking input
        let mut mtext_input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut mtext_input)?;
        let cstring = CString::new(mtext_input.trim()).unwrap();
        
        buff.mtext[..cstring.as_bytes().len()].copy_from_slice(cstring.as_bytes());

        println!("Please enter a number you want to send:");
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input)?;
        buff.number = number_input.trim().parse::<i32>().unwrap_or(0);

        unsafe {
            libc::msgsnd(msgid, &buff as *const _ as *const libc::c_void, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), 0);
        }

        // wait for the child process
        unsafe {
            libc::waitpid(pid, ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        // Child process
        let mut buff: MsgBuf = unsafe { mem::zeroed() };
        
        unsafe {
            libc::msgrcv(msgid, &mut buff as *mut _ as *mut libc::c_void, mem::size_of::<MsgBuf>() - mem::size_of::<i64>(), MY_TYPE, 0);
        }

        // Output the received message
        let received_str = unsafe { CString::from_raw(buff.mtext.as_mut_ptr()).into_string().unwrap() };
        printf!("Child process read from msg: {}, {}\n", received_str, buff.number);
    } else {
        eprintln!("fork failed");
    }

    Ok(())
}

// `printf` function as Rust equivalent
extern {
    fn printf(fmt: *const i8, ...) -> i32;
}

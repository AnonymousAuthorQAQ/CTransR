extern crate libc;

use std::ptr::null_mut;
use std::ffi::CString;
use std::{thread, process};
use libc::{c_long, c_int, size_t};

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: c_int,
}

fn main() {
    let proj_id: libc::key_t = 9527;
    let key = unsafe { libc::ftok(CString::new("/tmp").unwrap().as_ptr(), proj_id) };
    if key == -1 {
        eprintln!("ftok failed");
        process::exit(-1);
    }

    let msgid = unsafe { libc::msgget(key, 0o666 | libc::IPC_CREAT) };
    if msgid == -1 {
        eprintln!("msgget failed");
        process::exit(-1);
    }

    let mut pid;
    unsafe {
        pid = libc::fork();
    }

    if pid > 0 {
        thread::sleep(std::time::Duration::new(1, 0));

        let mut buffer: MsgBuf = MsgBuf {
            mtype: 9527,
            mtext: [0; 100],
            number: 0,
        };

        println!("Please enter a string you want to send:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop(); // Remove newline character
        let input_cstr = CString::new(input).unwrap();
        let input_bytes = input_cstr.as_bytes_with_nul();
        buffer.mtext[..input_bytes.len()].copy_from_slice(input_bytes);

        println!("Please enter a number you want to send:");
        let mut number_input = String::new();
        std::io::stdin().read_line(&mut number_input).unwrap();
        let number: i32 = number_input.trim().parse().unwrap();
        buffer.number = number;
        
        let nwritten = unsafe {
            libc::msgsnd(msgid, &buffer as *const MsgBuf as *mut libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), 0)
        };
        
        if nwritten == -1 {
            eprintln!("msgsend failed");
            process::exit(-1);
        }
        
        unsafe { libc::waitpid(pid, null_mut(), 0); }
    } else if pid == 0 {
        let mut buffer: MsgBuf = MsgBuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        println!("Child process is waiting for message:");
        let chunk_type = 9527;
        
        let nread = unsafe {
            libc::msgrcv(msgid, &buffer as *mut MsgBuf as *mut libc::c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>(), chunk_type, 0)
        };

        if nread == -1 {
            eprintln!("msgrcv failed");
            process::exit(-1);
        }

        let message = CString::from_vec_unchecked(buffer.mtext.iter().cloned().collect::<Vec<_>>());
        println!("Child process read from message: {}, {}", message.to_str().unwrap(), buffer.number);
    } else {
        eprintln!("Fork failed");
        process::exit(-1);
    }
}
extern crate libc;

use std::ptr;
use std::ffi::CString;

#[repr(C)]
struct msgbuf {
    mtype: libc::c_long,
    mtext: [libc::c_char; 100],
    number: libc::c_int,
}

fn main() {
    let msgid = unsafe {
        libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL)
    };

    if msgid == -1 {
        panic!("Failed to create message queue");
    }

    let mut buff: msgbuf = msgbuf {
        mtype: 9527,
        mtext: [0; 100],
        number: 0,
    };

    unsafe {
        let pid = libc::fork();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            println!("Please enter a string you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().as_bytes();
            ptr::copy_nonoverlapping(input.as_ptr() as *const libc::c_void, buff.mtext.as_mut_ptr() as *mut libc::c_void, input.len());

            println!("Please enter a number you want to send:");
            let mut number_input = String::new();
            std::io::stdin().read_line(&mut number_input).unwrap();
            let number_input: i32 = number_input.trim().parse().unwrap();
            buff.number = number_input;

            libc::msgsnd(msgid, &buff as *const msgbuf as *const libc::c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<libc::c_long>(), 0);

            libc::waitpid(pid, std::ptr::null_mut(), 0);
        } else if pid == 0 {
            println!("Child process is waiting for message:");

            libc::msgrcv(msgid, &mut buff as *mut msgbuf as *mut libc::c_void, std::mem::size_of::<msgbuf>() - std::mem::size_of::<libc::c_long>(), 9527, 0);

            let c_str = CString::from_vec_unchecked(buff.mtext.iter().map(|&c| c as u8).collect::<Vec<u8>>());
            let received_string = c_str.to_str().expect("Failed to convert into UTF-8 string");
            println!("Child process read from message: {}, {}", received_string, buff.number);
        } else {
            panic!("Fork failed");
        }
    }
}
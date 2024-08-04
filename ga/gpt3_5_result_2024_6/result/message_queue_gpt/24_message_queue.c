use std::mem;
use std::ffi::CString;
use std::os::raw::c_char;
use std::process;
use libc::{fork, getpid, waitpid, msgget, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL};

const MY_TYPE: i64 = 9527;

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error creating message queue");
        process::exit(1);
    }

    let pid = unsafe { fork() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff: [u8; mem::size_of::<libc::msqid_ds>() - mem::size_of::<libc::msgcid>()] = [0; 100];
        let c_string = CString::new("Hello from parent").unwrap();
        let message = c_string.as_bytes_with_nul();
        buff[..message.len()].copy_from_slice(message);

        let number: i32 = 123;
        msgsnd(msgid, &buff as *const u8 as *const libc::msqid_ds, message.len() as isize, 0);

        unsafe {
            waitpid(pid, std::ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buff: [u8; mem::size_of::<libc::msqid_ds>() - mem::size_of::<libc::msgcid>()] = [0; 100];
        msgrcv(msgid, &mut buff as *mut u8 as *mut libc::msqid_ds, mem::size_of::<libc::msqid_ds>(), MY_TYPE, 0);

        let c_str = CString::from_vec_unchecked(buff.to_vec());
        let received_message = c_str.to_str().unwrap_or("");

        println!("Child process read from msg: {}, {}", received_message, 123);
    } else {
        eprintln!("Error forking");
        process::exit(1);
    }
}
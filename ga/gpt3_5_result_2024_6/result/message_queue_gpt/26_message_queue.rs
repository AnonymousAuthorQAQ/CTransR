use std::process;
use libc::{c_long, c_void};
use std::ffi::{CString};
use std::mem;
use std::ptr;

const MY_TYPE: i64 = 9527;

fn main() {
    let msgid = unsafe { libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_EXCL) };

    if msgid == -1 {
        panic!("msgget failed");
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut buff: libc::msqid_ds = unsafe { mem::zeroed() };
        buff.msg_type = MY_TYPE;
        
        let mut input = String::new();
        println!("Please enter a string you want to send:");
        std::io::stdin().read_line(&mut input).unwrap();
        let input_len = input.trim().len();
        
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr() as *const c_void, buff.msg_text.as_mut_ptr(), input_len);
        }

        let number: i32;
        println!("Please enter a nubmer you want to send:");
        std::io::stdin().read_line(&mut input).unwrap();
        number = input.trim().parse().unwrap();
        
        libc::msgsnd(msgid, &mut buff as *mut libc::msqid_ds as *mut c_void, input_len as usize, 0);

        process::wait();
    } else if pid == 0 {
        let mut buff: libc::msqid_ds = unsafe { mem::zeroed() };
        
        libc::msgrcv(msgid, &mut buff as *mut libc::msqid_ds as *mut c_void, mem::size_of::<libc::msqid_ds>(), MY_TYPE, 0);
        
        let text: String = unsafe { 
            let mut text = String::new();
            let text_ptr = buff.msg_text.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect::<Vec<_>>();
            text.push_str(&String::from_utf8_lossy(&text_ptr));
            text
        };
        
        println!("Child process read from msg: {}, {}", text, buff.msg_type);
    } else {
        panic!("fork failed");
    }
}
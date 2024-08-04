use std::io::{self, Read};
use std::process::Command;
use std::ptr;
use std::ffi::CString;

fn main() {
    let msg_key = unsafe { libc::ftok("/etc/passwd", 'm' as i32) };
    let msg_id = unsafe { libc::msgget(msg_key, 0666 | libc::IPC_CREAT | libc::IPC_EXCL) };

    if msg_id == -1 {
        eprintln!("Failed to create message queue");
        return;
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        unsafe { libc::sleep(1) };

        let mut buff = libc::msgrcv(msg_id, ptr::null_mut(), 0, 0, 0);

        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        let input_text = CString::new(input_text.trim()).unwrap();
        let input_number = CString::new("0").unwrap();

        unsafe {
            libc::msgsnd(msg_id, input_text.as_ptr() as *const _, input_text.as_bytes().len() as i32, 0);
            libc::msgsnd(msg_id, input_number.as_ptr() as *const _, input_number.as_bytes().len() as i32, 0);
        }

        let status = Command::new("wait")
            .arg(pid.to_string())
            .status()
            .expect("Failed to wait for child process");

        if !status.success() {
            eprintln!("Child process did not exit successfully");
        }
    } else if pid == 0 {
        let mut buff = libc::msgrcv(msg_id, ptr::null_mut(), 0, 0, 0);

        let mut output_text = vec![0; 100];
        let mut output_number = 0;

        unsafe {
            libc::msgrcv(msg_id, output_text.as_mut_ptr() as *mut _, output_text.len() as i32, 0, 0);
            libc::msgrcv(msg_id, &mut output_number as *mut _, std::mem::size_of::<i32>() as i32, 0, 0);
        }

        println!("Child process read from msg: {}", output_text.iter().take_while(|&c| c != 0).collect::<String>());
        println!("Child process read from msg: {}", output_number);
    } else {
        eprintln!("Failed to fork process");
    }

    unsafe {
        libc::msgctl(msg_id, libc::IPC_RMID, ptr::null_mut());
    }
}
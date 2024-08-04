use std::io::{self, Read};
use std::os::unix::io::{FromRawFd, RawFd};
use std::process::{Command, Stdio};
use std::ptr;
use std::str;
use std::time::Duration;
use std::thread::sleep;

const MY_TYPE: i32 = 9527;

fn main() {
    let mut child_pid = 0;
    let msg_key = unsafe { libc::msgget(libc::IPC_PRIVATE, 0666 | libc::IPC_CREAT) };
    if msg_key == -1 {
        eprintln!("msgget failed");
        return;
    }

    child_pid = unsafe { libc::fork() };
    if child_pid == 0 {
        // Child process
        let mut msg_buf = libc::msgrcv(msg_key, ptr::null_mut(), 0, MY_TYPE, 0);
        if msg_buf.is_null() {
            eprintln!("msgrcv failed");
            unsafe {
                libc::msgctl(msg_key, libc::IPC_RMID, ptr::null_mut());
            }
            return;
        }
        let msg_text = unsafe { str::from_utf8_unchecked(&msg_buf[..]) };
        let msg_number = unsafe { *(msg_buf.add(100) as *const i32) };
        println!("Child process read from msg: {}, {}", msg_text, msg_number);
        unsafe {
            libc::msgctl(msg_key, libc::IPC_RMID, ptr::null_mut());
        }
    } else if child_pid > 0 {
        // Parent process
        sleep(Duration::from_secs(1));
        println!("Please enter a string you want to send:");
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).expect("Failed to read line");
        let input_text = input_text.trim_end_matches('\n');

        println!("Please enter a number you want to send:");
        let mut input_number = String::new();
        io::stdin().read_line(&mut input_number).expect("Failed to read line");
        let input_number: i32 = input_number.trim().parse().expect("Failed to parse number");

        let msg = format!("{} {}", input_text, input_number);
        if unsafe { libc::msgsnd(msg_key, msg.as_ptr() as *const libc::c_void, msg.len(), 0) } == -1 {
            eprintln!("msgsnd failed");
        }

        let status = unsafe { libc::waitpid(child_pid, ptr::null_mut(), 0) };
        if status == -1 {
            eprintln!("waitpid failed");
        }
    } else {
        eprintln!("fork failed");
    }
}
use std::ffi::CString;
use std::ptr;
use std::process::exit;
use std::ptr::null;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let child_pid = unsafe {
        match fork() {
            0 => {
                // Child process
                let msg = receiver.recv().unwrap();
                println!("Child process read from msg: {}", msg);
                exit(0);
            }
            pid => pid,
        }
    };

    // Parent process
    thread::sleep(Duration::from_secs(1));
    let msg = CString::new("Hello, World!").unwrap();
    sender.send(msg.to_str().unwrap()).unwrap();

    unsafe {
        waitpid(child_pid, null_mut(), 0);
    }
}
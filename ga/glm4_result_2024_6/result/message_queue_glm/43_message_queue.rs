use std::io::{self, Read};
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let child_pid = unsafe {
        match fork() {
            Ok(0) => {
                // Child process
                let received = rx.recv().unwrap();
                println!("Child process read from channel: {}, {}", received.0, received.1);
                exit(0);
            }
            Ok(pid) => {
                // Parent process
                thread::sleep(Duration::from_secs(1));
                print!("Please enter a string you want to send: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut number: i32 = 0;
                print!("Please enter a number you want to send: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                input.trim().parse::<i32>().unwrap();
                tx.send((input.trim().to_string(), number)).unwrap();
                waitpid(pid, None).unwrap();
                exit(0);
            }
            Err(e) => {
                eprintln!("fork failed: {}", e);
                exit(1);
            }
        }
    };
}

fn fork() -> Result<pid_t, io::Error> {
    use std::ffi::CString;
    use std::os::unix::prelude::*;

    unsafe {
        let pid = libc::fork();
        if pid == 0 {
            Ok(0)
        } else if pid > 0 {
            Ok(pid as pid_t)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

fn waitpid(pid: pid_t, options: Option<libc::c_int>) -> Result<pid_t, io::Error> {
    use std::os::unix::prelude::*;

    unsafe {
        let pid = libc::waitpid(pid as i32, options.unwrap_or(0));
        if pid == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(pid as pid_t)
        }
    }
}
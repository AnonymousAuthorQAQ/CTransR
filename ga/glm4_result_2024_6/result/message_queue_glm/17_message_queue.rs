use std::ffi::CString;
use std::io::{self, Write};
use std::ptr;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let pid = unsafe { libc::fork() };

    if pid > 0 {
        thread::sleep(Duration::from_secs(1));

        let mut input_text = String::new();
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_text).expect("Failed to read line");

        let number: i32 = loop {
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse() {
                Ok(num) => break num,
                Err(_) => println!("Invalid number, please try again."),
            }
        };

        sender.send((input_text.trim().to_string(), number)).unwrap();
        unsafe { libc::waitpid(pid, ptr::null_mut(), 0); }
    } else if pid == 0 {
        let (text, number) = receiver.recv().unwrap();
        println!("Child process read from msg: {}, {}", text, number);
    } else {
        eprintln!("fork failed");
        exit(1);
    }
}
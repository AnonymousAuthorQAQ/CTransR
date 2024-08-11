use std::io::{self, Write};
use std::process::Command;
use sysv_ipc::{MessageQueue, Key, Result};
use std::os::unix::process::CommandExt;
use std::ffi::CString;

const MY_TYPE: u64 = 9527;

fn main() -> Result<()> {
    let msg_key = Key::new(12345)?;
    let msg_queue = MessageQueue::new(msg_key, 0)?;

    let pid = unsafe { libc::fork() };

    if pid < 0 {
        eprintln!("Fork failed");
        return Ok(());
    } else if pid > 0 {
        // Parent process
        let mut input_text = String::new();
        let mut input_number = String::new();

        println!("Please enter a string you want to send:");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");
        input_text = input_text.trim().to_string();

        println!("Please enter a number you want to send:");
        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read line");
        let number = input_number.trim().parse::<i32>().unwrap();

        let mut msg_text = input_text.clone();
        msg_text.push('\0'); // Add null terminator to mimic C string behavior

        msg_queue.send(MY_TYPE, &msg_text.as_bytes())?;
        msg_queue.send(MY_TYPE, &format!("{}", number).into_bytes())?;

        unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
    } else {
        // Child process
        let (msg_type, msg) = msg_queue.receive(MY_TYPE)?;
        let msg_str = String::from_utf8_lossy(&msg);
        
        println!("Child process read from msg: {}", msg_str);
    }

    Ok(())
}
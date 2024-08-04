use std::io::{self, Write};
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use msg::{MsgQueue, MsgQueueError};

const MY_TYPE: i64 = 9527;

fn main() -> Result<(), MsgQueueError> {
    let msg_queue = MsgQueue::new(0, 0o666 | IPC_CREAT)?;

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        let mut input_text = String::new();
        let mut number: i32 = 0;

        // Sleep for a bit to let the child prepare
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Reading string input from user
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_text).unwrap();
        let trimmed_text = input_text.trim();

        // Reading number input from user
        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input).unwrap();
        number = number_input.trim().parse().unwrap();

        // Send the message
        let message = format!("{}{}", trimmed_text, number);
        msg_queue.send(MY_TYPE, message.as_bytes())?;
        
        // Wait for child to finish
        unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let mut buffer = [0u8; 100];
        let (msg_type, size) = msg_queue.recv(&mut buffer, MY_TYPE)?;

        // Convert the received message bytes to string
        let received_text = String::from_utf8_lossy(&buffer[..size]);
        
        // Print the received message
        println!("Child process read from msg: {}", received_text);
    } else {
        eprintln!("Failed to fork process");
        exit(1);
    }

    Ok(())
}
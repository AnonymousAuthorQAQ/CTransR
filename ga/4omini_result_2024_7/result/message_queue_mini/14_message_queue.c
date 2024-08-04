use std::io::{self, Write};
use std::process::{Command, exit};
use std::thread;
use ipc_channel::ipc::{self, IpcReceiver, IpcSender};

fn main() {
    // Create a channel for inter-process communication
    let (tx, rx): (IpcSender<(String, i32)>, IpcReceiver<(String, i32)>) = ipc::channel().unwrap();

    let pid = unsafe { libc::fork() };

    match pid {
        -1 => {
            eprintln!("Fork failed");
            exit(1);
        }
        0 => { // Child process
            // Receive the message from the parent
            let (text, number) = rx.recv().unwrap();
            println!("Child process read from msg: {}, {}", text, number);
        }
        _ => { // Parent process
            // Sleep for a moment to ensure the child is ready to receive
            thread::sleep(std::time::Duration::from_secs(1));
            
            let mut input_text = String::new();
            let mut input_number = String::new();

            // Reading a string from the user
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_text).unwrap();
            let input_text = input_text.trim().to_string();

            // Reading a number from the user
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_number).unwrap();
            let input_number: i32 = input_number.trim().parse().unwrap();

            // Send the data to the child
            tx.send((input_text, input_number)).unwrap();

            // Wait for the child process to finish before exiting
            let _ = unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
        }
    }
}
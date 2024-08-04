use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::process::CommandExt;
use std::ffi::CString;

const MY_TYPE: u32 = 9527;

fn main() -> io::Result<()> {
    // Create a pipe for communication
    let (mut reader, mut writer) = os_pipe::pipe()?;
    
    let pid = unsafe {
        // Fork the process
        let child = Command::new("child")
            .stdin(Stdio::from_raw_fd(writer.as_raw_fd()))
            .stdout(Stdio::inherit())
            .spawn()?;
        child.id()
    };

    // Sleep to ensure the child process is ready (not an ideal way, just for simplicity)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Parent process gathers input
    let mut buffer = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut buffer)?;
    let string_input = buffer.trim(); // Remove any trailing newline
    println!("Please enter a number you want to send:");
    
    // Read the integer input
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input)?;
    let number: i32 = number_input.trim().parse().expect("Please enter a valid number.");

    // Send data to the child process
    writeln!(writer, "{} {}", string_input, number)?;

    // Wait for the child process to finish
    let _ = std::process::Command::new("wait").arg(pid.to_string()).output();

    Ok(())
}
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    for line in handle.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let string_input = parts[0];
            let number_input: i32 = parts[1].parse().expect("Invalid number format");
            println!("Child process read from msg: {}, {}", string_input, number_input);
        }
    }

    Ok(())
}
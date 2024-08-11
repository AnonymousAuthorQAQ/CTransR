use std::io::{self, Read};
use std::process::Command;
use std::ptr::null;
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::channel;

const MY_TYPE: i32 = 9527;

#[derive(Debug)]
struct MsgBuf {
    mtype: i32,
    mtext: String,
    number: i32,
}

fn main() {
    let (sender, receiver) = channel();

    let child_pid = match unsafe { Command::new("rust_child_process").spawn() } {
        Ok(child) => {
            println!("Child process started with PID {}", child.id());
            child.id()
        }
        Err(e) => {
            eprintln!("Failed to start child process: {}", e);
            return;
        }
    };

    // Parent process
    println!("Enter a string to send:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mtext = input.trim().to_string();

    println!("Enter a number to send:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number = number.trim().parse::<i32>().expect("Failed to parse number");

    let msg = MsgBuf {
        mtype: MY_TYPE,
        mtext: mtext,
        number: number,
    };
    sender.send(msg).expect("Failed to send message");

    sleep(Duration::from_secs(1));

    let mut output = String::new();
    io::stdin().read_line(&mut output).expect("Failed to read line");
    println!("Parent received: {}", output.trim());

    let _ = Command::new("kill").args(&[child_pid.to_string().as_str(), "SIGKILL"]).status();
}

fn main_child() {
    let (sender, receiver) = channel();

    let msg = receiver.recv().expect("Failed to receive message");
    println!("Child received: {:?}", msg);

    let output = "Hello from child".to_string();
    sender.send(output).expect("Failed to send response");
}

#[cfg(target_os = "linux")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "child" {
        main_child();
    } else {
        main();
    }
}
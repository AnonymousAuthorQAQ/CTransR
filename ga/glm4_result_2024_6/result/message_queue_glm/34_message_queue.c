use std::io::{self, Read};
use std::process::Command;
use ipc_channel::ipc;

fn main() {
    let (sender, receiver) = ipc::channel().unwrap();

    let child_pid = match unsafe { Command::new("cargo").args(&["run", "--bin", "child"]).spawn() } {
        Ok(child) => child.id(),
        Err(e) => {
            eprintln!("Failed to spawn child process: {}", e);
            return;
        }
    };

    println!("Please enter a string you want to send:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let input_string = input_string.trim().to_string();

    println!("Please enter a number you want to send:");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let input_number: i32 = input_number.trim().parse().expect("Please enter a valid number");

    sender.send((input_string, input_number)).unwrap();

    let received = receiver.recv().unwrap();
    println!("Parent received: {}", received.0);
    println!("Parent received: {}", received.1);

    let status = Command::new("kill").args(&[child_pid.to_string().as_str()]).status().expect("Failed to kill child process");
    if !status.success() {
        eprintln!("Failed to kill child process");
    }
}

// Child process
fn child() {
    let (sender, receiver) = ipc::channel().unwrap();

    let received = receiver.recv().unwrap();
    println!("Child received: {}", received.0);
    println!("Child received: {}", received.1);

    sender.send(received).unwrap();
}

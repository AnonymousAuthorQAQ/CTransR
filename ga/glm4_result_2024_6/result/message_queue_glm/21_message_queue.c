use std::io::{self, Read};
use std::process::Command;
use ipc_channel::ipc::IpcSender;
use ipc_channel::router::Router;
use ipc_channel::concrete::Concrete;

fn main() {
    let (sender, receiver) = Concrete::new().unwrap();
    let mut child = Command::new("cargo")
        .args(&["run", "--example", "child_process"])
        .spawn()
        .expect("Failed to spawn child process");

    // Wait for the child process to start
    std::thread::sleep(std::time::Duration::from_secs(1));

    let mut input_text = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let input_text = input_text.trim();

    let mut input_number = String::new();
    println!("Please enter a number you want to send:");
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let input_number: i32 = input_number.trim().parse().expect("Invalid number");

    sender.send((input_text.to_string(), input_number)).unwrap();

    let result = receiver.recv().unwrap();
    println!("Received from child: {:?}", result);

    let status = child.wait().expect("Failed to wait on child");
    assert!(status.success());
}

#[cfg(target_os = "linux")]
fn main() {
    // This is the main function for the child process
    let (sender, receiver) = Concrete::new().unwrap();
    let result = receiver.recv().unwrap();
    println!("Child process read from msg: {}, {}", result.0, result.1);
    sender.send(result).unwrap();
}
use std::io::{self, Read};
use std::process::Command;
use crossbeam::crossbeam_channel::{unbounded, Sender, Receiver};

fn main() {
    let (sender, receiver) = unbounded();

    let child = Command::new("rust_child")
        .spawn()
        .expect("Failed to spawn child process");

    let mut input = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    println!("Please enter a number you want to send:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Failed to parse number");

    sender.send((input.to_string(), number)).expect("Failed to send message");

    let status = child.wait_with_output().expect("Failed to wait for child");
    println!("Child process output:\n{}", String::from_utf8_lossy(&status.stdout));
}

fn main_child() {
    let (sender, receiver) = unbounded();

    let (input, number) = receiver.recv().expect("Failed to receive message");
    println!("Child process read from msg: {}, {}", input, number);
}

#[cfg(target_os = "linux")]
#[link(name = "c")]
extern "C" {
    fn fork() -> i32;
}

#[cfg(target_os = "linux")]
fn fork() -> i32 {
    unsafe { fork() }
}

#[cfg(not(target_os = "linux"))]
fn fork() -> i32 {
    panic!("Fork is not supported on this platform");
}
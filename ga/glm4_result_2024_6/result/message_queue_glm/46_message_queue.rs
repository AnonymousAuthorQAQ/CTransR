use std::io::{self, Read};
use std::process::Command;
use std::os::unix::process::CommandExt;

const MY_TYPE: i32 = 9527;

fn main() {
    let mut input_text = String::new();
    let mut input_number = String::new();

    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    println!("Please enter a number you want to send:");
    io::stdin().read_line(&mut input_number).expect("Failed to read line");

    let input_number: i32 = input_number.trim().parse().expect("Please type a number!");

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("echo '{:?}' | msgget {}", input_text.trim(), MY_TYPE))
        .spawn()
        .expect("Failed to spawn child process");

    let status = child.wait().expect("Failed to wait on child");
    println!("Child process exited with: {}", status);
}
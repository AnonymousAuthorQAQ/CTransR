use std::io::{self, Read};
use std::process::{self, Child, Command};
use std::thread;
use std::time::Duration;

const MY_TYPE: i32 = 9527;

#[derive(Debug)]
struct MsgBuf {
    mtype: i32,
    mtext: String,
    number: i32,
}

fn main() {
    let mut input = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mtext: String = input.trim().to_string();

    println!("Please enter a number you want to send:");
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read line");
    let number: i32 = number_input.trim().parse().expect("Failed to parse number");

    let msg = MsgBuf {
        mtype: MY_TYPE,
        mtext,
        number,
    };

    let mut child = Command::new("sh")
        .args(&["-c", "echo $MSG | tr -d '\n' | sed 's/[^0-9]//g' | wc -c"])
        .spawn()
        .expect("Failed to spawn child process");

    thread::sleep(Duration::from_secs(1));

    println!("Parent process sending message: {:?}", msg);
    let output = child
        .wait_with_output()
        .expect("Failed to wait on child process");

    println!("Child process output: {}", String::from_utf8_lossy(&output.stdout));
}
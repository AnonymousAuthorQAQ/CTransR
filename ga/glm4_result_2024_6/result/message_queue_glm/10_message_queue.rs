use std::io::{self, Read};
use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    let mut input_text = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let input_text = input_text.trim_end();

    let mut input_number = String::new();
    println!("Please enter a number you want to send:");
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let input_number: i32 = input_number.trim_end().parse().expect("Please enter a valid number");

    let child_pid = unsafe {
        Command::new("sh")
            .args(&["-c", "echo $MSG_Q_ID $MSG_TYPE $input_text $input_number"])
            .spawn()
            .expect("Failed to spawn child process")
            .pid
    };

    println!("Parent process is waiting for child to finish");
    let status = Command::new("wait").arg(child_pid).status().expect("Failed to wait for child process");
    println!("Child process finished with status: {}", status);
}

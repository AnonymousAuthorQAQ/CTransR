use std::io::{self, Read};
use std::process::{self, Child, Command};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let child = process::Command::new("rustc")
        .args(&["--version"])
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    thread::spawn(move || {
        let mut output = String::new();
        child.stdout.unwrap().read_to_string(&mut output).unwrap();
        tx.send(output).unwrap();
    });

    thread::sleep(Duration::from_secs(1));

    let output = rx.recv().unwrap();
    println!("Child process output:\n{}", output);
}
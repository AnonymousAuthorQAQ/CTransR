use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::str;
use std::thread;
use std::time::Duration;

const PORT: u16 = 9527;
const BUF_SIZE: usize = 1024;

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", PORT)).expect("Failed to bind");
    let mut stream = listener.incoming().next().expect("Failed to accept").expect("Failed to get stream");

    let mut child_pid = Command::new("rust_child")
        .spawn()
        .expect("Failed to spawn child");

    thread::sleep(Duration::from_secs(1));

    let mut buffer = [0; BUF_SIZE];
    println!("Please enter a string you want to send:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut string_to_send = input.trim().to_string() + "\n";

    println!("Please enter a number you want to send:");
    let mut number_to_send = String::new();
    std::io::stdin().read_line(&mut number_to_send).expect("Failed to read line");
    let number_to_send: i32 = number_to_send.trim().parse().expect("Failed to parse number");

    stream.write_all(string_to_send.as_bytes()).expect("Failed to write");
    stream.write_all(number_to_send.to_string().as_bytes()).expect("Failed to write");

    let mut received = String::new();
    stream.read_to_string(&mut received).expect("Failed to read");
    println!("Received from child: {}", received.trim());

    child_pid.wait().expect("Failed to wait for child");
}

fn main_child() {
    let stream = TcpStream::connect(("127.0.0.1", PORT)).expect("Failed to connect");

    let mut buffer = [0; BUF_SIZE];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read");
    let received = str::from_utf8(&buffer[..bytes_read]).expect("Failed to parse bytes");

    println!("Child process read from msg: {}", received.trim());
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This example is designed for Linux and will not run on other operating systems.");
}
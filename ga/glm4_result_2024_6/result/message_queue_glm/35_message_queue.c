use std::io::{self, Read};
use std::process::Command;
use std::thread;
use std::time::Duration;

mod ipc_channel;

fn main() {
    let (sender, receiver) = ipc_channel::new();
    
    let sender_clone = sender.clone();
    
    let handle = thread::spawn(move || {
        let mut input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        println!("Please enter a number you want to send:");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Please enter a valid number");
        
        sender_clone.send((input.to_string(), number)).expect("Failed to send message");
    });
    
    thread::sleep(Duration::from_secs(1));
    
    if let Ok((text, num)) = receiver.recv() {
        println!("Child process read from msg: {}, {}", text, num);
    } else {
        println!("Child process failed to receive message");
    }
    
    handle.join().expect("Failed to join thread");
}
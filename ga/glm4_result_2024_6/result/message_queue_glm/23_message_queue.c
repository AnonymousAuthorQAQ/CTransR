use std::io::{self, Read};
use std::process::exit;
use std::thread;
use std::time::Duration;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::IpcSender;
use ipc_channel::ipc::IpcSelectionResult;
use ipc_channel::platform::IpcOneShotServer;
use ipc_channel::platform::IpcReceiverSet;

fn main() {
    let (tx, rx): (IpcSender<(String, i32)>, IpcReceiver<(String, i32)>) = IpcOneShotServer::new().unwrap().into通道();
    let tx2 = tx.clone();

    let child_thread = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(data) => {
                    println!("Child process read from msg: {}, {}", data.0, data.1);
                    break;
                }
                Err(IpcSelectionResult::NoMessage) => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    exit(1);
                }
            }
        }
    });

    thread::sleep(Duration::from_secs(1));

    let mut input_text = String::new();
    println!("Please enter a string you want to send:");
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let input_text = input_text.trim().to_string();

    println!("Please enter a number you want to send:");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let input_number: i32 = input_number.trim().parse().expect("Invalid number");

    tx2.send((input_text, input_number)).expect("Failed to send message");

    child_thread.join().expect("Failed to join child thread");
}
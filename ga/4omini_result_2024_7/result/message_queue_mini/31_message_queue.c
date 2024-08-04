use nix::sys::msg::{MsgQueue, Msg};
use nix::unistd::{fork, ForkResult};
use std::io::{self, Write};

const MY_TYPE: i64 = 9527;

#[derive(Debug)]
struct Message {
    mtype: i64,
    mtext: String,
    number: i32,
}

fn main() -> nix::Result<()> {
    // Create a message queue
    let msg_queue = MsgQueue::new(None)?;

    // Fork the process
    match fork()? {
        ForkResult::Parent { child } => {
            // Parent process
            std::thread::sleep(std::time::Duration::from_secs(1));

            // Input string
            let mut mtext = String::new();
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap(); // Flush stdout to ensure prompt is displayed
            io::stdin().read_line(&mut mtext).unwrap();
            let mtext = mtext.trim().to_string(); // Remove trailing newline

            // Input number
            let mut number_str = String::new();
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut number_str).unwrap();
            let number: i32 = number_str.trim().parse().unwrap();

            // Create and send message
            let msg = Message { mtype: MY_TYPE, mtext, number };
            msg_queue.send(&msg)?;

            // Wait for the child process
            nix::sys::wait::waitpid(child, None)?;
        }
        ForkResult::Child => {
            // Child process
            let msg: Message = msg_queue.receive(MY_TYPE)?;

            // Output received message
            println!("Child process read from msg: {}, {}", msg.mtext, msg.number);
        }
    }

    Ok(())
}
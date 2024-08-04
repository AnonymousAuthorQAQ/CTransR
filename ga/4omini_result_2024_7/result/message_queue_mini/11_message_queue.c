use msg_queue::{MsgQueue, Msg};
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Message {
    mtype: u32,
    mtext: String,
    number: i32,
}

fn main() {
    let msg_queue = MsgQueue::new().expect("Failed to create message queue");
    let my_type: u32 = 9527;

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        thread::sleep(Duration::from_secs(1)); // Wait for a while

        let mut input_text = String::new();
        let mut input_number = String::new();

        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_text).expect("Failed to read line");
        let input_text = input_text.trim().to_string();

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_number).expect("Failed to read line");
        let input_number: i32 = input_number.trim().parse().expect("Please enter a valid number");

        // Send message
        let msg = Message {
            mtype: my_type,
            mtext: input_text,
            number: input_number,
        };

        msg_queue.send(Msg::new(msg.mtype, msg)).expect("Failed to send message");

        // Wait for the child process to finish
        unsafe { libc::waitpid(pid, std::ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let msg: Msg<Message> = msg_queue.recv(my_type).expect("Failed to receive message");
        let received: Message = msg.data;

        println!("Child process read from msg: {}, {}", received.mtext, received.number);
    } else {
        eprintln!("Fork failed");
        process::exit(1);
    }
}
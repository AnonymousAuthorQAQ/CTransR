use nix::errno::Errno;
use nix::fcntl::OFlag;
use nix::ipc::msg::{MsgBuf, MsgQueue, MsgFlags};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::ptr;
use std::ptr::null_mut;

const MY_TYPE: i64 = 9527;

#[derive(Debug)]
struct MyMsg {
    mtype: i64,
    mtext: Vec<u8>,
    number: i32,
}

impl MyMsg {
    fn new(mtype: i64, mtext: Vec<u8>, number: i32) -> Self {
        MyMsg { mtype, mtext, number }
    }
}

fn main() {
    let msgid = MsgQueue::create().expect("Failed to create message queue"); // Create a new message queue

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process 
        let mut mtext = vec![0u8; 100]; // buffer for the message text
        println!("Please enter a string you want to send:");
        io::stdout().flush().unwrap();
        io::stdin().read(&mut mtext).unwrap();
        let input_string = String::from_utf8_lossy(&mtext);
        let trimmed_string = input_string.trim();

        println!("Please enter a number you want to send:");
        let mut number_string = String::new();
        io::stdin().read_line(&mut number_string).unwrap();
        let number: i32 = number_string.trim().parse().expect("Please enter a valid number");

        let msg = MyMsg::new(MY_TYPE, trimmed_string.into_bytes(), number);
        
        if let Err(err) = msgid.send(&msg) {
            eprintln!("msgsnd failed: {}", err);
        }

        unsafe { libc::waitpid(pid, null_mut(), 0) }; // wait for child
    } else if pid == 0 {
        // Child process
        let mut msg = MyMsg::new(0, Vec::new(), 0);

        if let Ok(received_msg) = msgid.recv::<MyMsg>(MsgFlags::empty()) {
            msg = received_msg; // Get the received message
            // print the message obtained
            let received_text = String::from_utf8_lossy(&msg.mtext);
            println!("Child process read from msg: {}, {}", received_text, msg.number);
        } else {
            eprintln!("msgrcv failed");
        }
    } else {
        eprintln!("fork failed: {}", Errno::last());
        exit(1);
    }
}
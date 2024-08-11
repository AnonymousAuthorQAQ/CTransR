use std::io::{self, Read};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::str;

use ipc::MessageQueue;

fn main() {
    let msg_queue = MessageQueue::new().expect("Failed to create message queue");

    match fork() {
        Ok(0) => {
            // Child process
            let msg = msg_queue.receive().expect("Failed to receive message");
            println!("Child process read from msg: {}, {}", msg.text, msg.number);
        }
        Ok(pid) => {
            // Parent process
            let mut input = String::new();
            println!("Please enter a string you want to send:");
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let text = input.trim();

            println!("Please enter a number you want to send:");
            let mut number = String::new();
            io::stdin().read_line(&mut number).expect("Failed to read input");
            let number = number.trim().parse().expect("Invalid number");

            msg_queue.send(text, number).expect("Failed to send message");

            waitpid(pid, None).expect("Failed to wait for child");
        }
        Err(e) => {
            eprintln!("Failed to fork: {}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
struct Message {
    text: String,
    number: i32,
}

struct MessageQueue {
    id: i32,
}

impl MessageQueue {
    fn new() -> Result<MessageQueue, Box<dyn std::error::Error>> {
        let id = unsafe { libc::msgget(libc::IPC_PRIVATE, 0o666 | libc::IPC_CREAT | libc::IPC_EXCL) };
        if id < 0 {
            return Err("Failed to create message queue".into());
        }
        Ok(MessageQueue { id })
    }

    fn send(&self, text: &str, number: i32) -> Result<(), Box<dyn std::error::Error>> {
        let msg = format!("{}\0{}", text, number);
        unsafe {
            libc::msgsnd(self.id, msg.as_ptr() as *const _, msg.len(), 0);
        }
        Ok(())
    }

    fn receive(&self) -> Result<Message, Box<dyn std::error::Error>> {
        let mut buf = vec![0; 1024];
        let mut msg_type: libc::c_long = 0;
        let len = unsafe {
            libc::msgrcv(self.id, buf.as_mut_ptr() as *mut _, buf.len(), msg_type, 0)
        };
        if len < 0 {
            return Err("Failed to receive message".into());
        }
        let msg = unsafe { str::from_utf8_unchecked(&buf[..len]) };
        let parts: Vec<&str> = msg.split('\0').collect();
        let text = parts[0];
        let number = parts[1].parse().expect("Invalid number");
        Ok(Message { text: text.to_string(), number })
    }
}

fn fork() -> Result<i32, std::io::Error> {
    match unsafe { Command::new("/bin/echo").arg("fork").spawn()? } {
        Ok(mut child) => {
            let status = child.wait()?;
            Ok(status.code().unwrap_or(0))
        }
        Err(e) => Err(e),
    }
}

fn waitpid(pid: i32, options: Option<libc::c_int>) -> Result<libc::c_int, std::io::Error> {
    let status = unsafe { libc::waitpid(pid, options.unwrap_or(0), 0) };
    if status < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(status)
    }
}
use std::io::{self, Write};
use std::process::{Command, exit};
use std::ptr;
use std::{thread, time};
use sysv_ipc::{MsgQueue, Flags};

const MY_TYPE: i32 = 9527;

#[derive(Debug)]
struct MsgBuf {
    mtype: i32,
    mtext: String,
    number: i32,
}

fn main() -> io::Result<()> {
    let msg_queue = MsgQueue::new(None, Flags::IPC_CREAT | Flags::IPC_EXCL | Flags::S_IRUSR | Flags::S_IWUSR)?;

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        thread::sleep(time::Duration::from_secs(1));

        let mut mtext = String::new();
        let mut number = String::new();

        print!("Please enter a string you want to send: ");
        io::stdout().flush()?; // Flush stdout to ensure prompt appears before input
        io::stdin().read_line(&mut mtext)?;
        print!("Please enter a number you want to send: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut number)?;

        let number: i32 = number.trim().parse().expect("Please enter a valid number");

        let msg = MsgBuf {
            mtype: MY_TYPE,
            mtext: mtext.trim().to_string(),
            number,
        };

        msg_queue.send(&msg)?;

        unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let mut buf = MsgBuf {
            mtype: 0,
            mtext: String::new(),
            number: 0,
        };

        msg_queue.receive(&mut buf)?;

        println!("Child process read from msg: {}, {}", buf.mtext, buf.number);
    } else {
        eprintln!("Fork failed");
        exit(1);
    }

    Ok(())
}

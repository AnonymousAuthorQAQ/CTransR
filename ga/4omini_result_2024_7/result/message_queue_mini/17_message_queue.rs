use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::ptr;
use std::mem;
use std::ffi::CString;
use msg::{MsgQueue, MsgQueueError};

const MY_TYPE: i64 = 9527;

#[repr(C)]
#[derive(Debug)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() -> Result<(), MsgQueueError> {
    let msg_queue = MsgQueue::new(0)?; // IPC_PRIVATE is 0 in this context

    // Fork a child process
    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        let mut buff = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        println!("Please enter a string you want to send:");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.truncate(99); // Limit to 99 chars

        buff.mtext[..input.len()].copy_from_slice(input.as_bytes());

        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut input).unwrap();
        buff.number = input.trim().parse().unwrap_or(0);

        // Send the message to the queue
        msg_queue.send(&buff, mem::size_of::<MsgBuf>() as isize)?;

        // Wait for the child process to finish
        let _ = unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let mut buff = MsgBuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        // Receive the message from the queue
        msg_queue.recv(&mut buff, mem::size_of::<MsgBuf>() as isize)?;

        // Convert mtext to String for printing
        let received_text = String::from_utf8_lossy(&buff.mtext);
        println!("Child process read from msg: {}, {}", received_text.trim(), buff.number);
    } else {
        // Error handling for fork
        eprintln!("Fork failed");
    }

    Ok(())
}
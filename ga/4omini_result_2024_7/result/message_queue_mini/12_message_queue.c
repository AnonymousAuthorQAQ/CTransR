use nix::sys::msg::{msgsend, msgrcv, MsgBuffer, MsgFlags, Message};
use nix::unistd::{fork, ForkFlag, wait, Sleep};
use nix::errno::Errno;
use std::io::{self, Write};
use std::ffi::CString;

const MY_TYPE: i32 = 9527;

#[derive(Message)]
struct MsgBuf {
    mtype: i64,
    mtext: [i8; 100],
    number: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let msgid = nix::sys::msg::msgget(0, nix::sys::msg::ipc::IPC_CREAT | 0o666)?;
    
    match fork()? {
        // Parent process
        nix::unistd::ForkResult::Parent { child } => {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff: MsgBuf = unsafe { std::mem::zeroed() };

            // Enter a string
            print!("Please enter a string you want to send: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut buff.mtext)?;
            let trimmed = buff.mtext.iter().take_while(|&&x| x != 0).count();
            buff.mtext[trimmed] = 0; // null-terminate

            // Enter a number
            print!("Please enter a number you want to send: ");
            io::stdout().flush()?;
            let mut num_input = String::new();
            io::stdin().read_line(&mut num_input)?;
            buff.number = num_input.trim().parse::<i32>()?;

            buff.mtype = MY_TYPE as i64;

            msgsend(msgid, &buff, MsgFlags::empty())?;
            wait()?;
        }
        // Child process
        nix::unistd::ForkResult::Child => {
            let mut buff: MsgBuf = unsafe { std::mem::zeroed() };
            msgrcv(msgid, &mut buff, MsgFlags::empty())?;

            // Convert mtext back to a Rust string
            let message = unsafe { CString::from_raw(buff.mtext.as_mut_ptr()).to_string_lossy() };
            println!("Child process read from msg: {}, {}", message, buff.number);
        }
    }

    Ok(())
}
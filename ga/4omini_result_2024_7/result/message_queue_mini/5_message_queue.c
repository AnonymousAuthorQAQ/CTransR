use nix::sys::msg::*;
use nix::unistd::{fork, ForkType, sleep, wait};
use std::io::{self, Write};

const MY_TYPE: i64 = 9527;

#[derive(Debug)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    // Set up the message queue
    let msgid = MsgQueue::new().unwrap();

    // Fork the process
    match fork() {
        Ok(ForkType::Parent { child }) => {
            sleep(1);

            let mut buf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Read a string input
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buf.mtext.iter_mut().take(100).collect::<Vec<_>>()).unwrap();
            let length = buf.mtext.iter().position(|&x| x == 0).unwrap_or(100);
            msgsnd(msgid, &buf, length as usize - std::mem::size_of::<i64>()).unwrap();
            
            wait().unwrap();
        }
        Ok(ForkType::Child) => {
            let mut buf = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            // Receive message
            msgrcv(msgid, &mut buf, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>()).unwrap();
            println!(
                "Child process read from msg: {}, {}",
                String::from_utf8_lossy(&buf.mtext),
                buf.number
            );
        }
        Err(err) => {
            eprintln!("fork error: {}", err);
        }
    }
}

// Custom message queue setup some other function...
impl MsgQueue {
    fn new() -> Result<i32, std::io::Error> {
        let ftok = IPC_PRIVATE;
        let msgid = msgget(ftok, libc::IPC_CREAT | 0o666).map_err(|_| std::io::Error::last_os_error())?;
        Ok(msgid)
    }
}
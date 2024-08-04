use std::io::{self, Read};
use std::ptr;
use std::str;
use std::time::Duration;
use std::ffi::CString;
use ipc::{Msg, MsgBuf, MsgId, MsgPerm, IPC_PRIVATE};
use nix::unistd::{fork, ForkResult};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::sys::signal;

fn main() {
    let msgid = MsgId::new(IPC_PRIVATE, MsgPerm::default()).expect("msgget failed");
    
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            // Parent process
            std::thread::sleep(Duration::from_secs(1));
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("read_line failed");
            let input = input.trim_end_matches('\n');
            
            let mut number = 0;
            io::stdin().read_line(&mut input).expect("read_line failed");
            number = input.trim_end_matches('\n').parse().expect("parse failed");
            
            let msg = Msg {
                mtype: 9527,
                mtext: CString::new(input).unwrap().into_bytes_with_nul(),
                number,
            };
            
            msgsnd(msgid, &msg).expect("msgsnd failed");
            
            waitpid(child, None).expect("waitpid failed");
        }
        Ok(ForkResult::Child) => {
            // Child process
            let mut msgbuf = MsgBuf::new(100, 9527);
            msgrcv(msgid, &mut msgbuf).expect("msgrcv failed");
            
            let text = str::from_utf8(&msgbuf.mtext).unwrap().trim_end_matches('\0');
            let number = msgbuf.number;
            
            println!("Child process read from msg: {}, {}", text, number);
        }
        Err(e) => {
            eprintln!("fork failed: {}", e);
        }
    }
}

fn msgsnd(id: MsgId, msg: &Msg) -> Result<(), Box<dyn std::error::Error>> {
    let flags = 0;
    ipc::msgsnd(id, msg, flags)?;
    Ok(())
}

fn msgrcv(id: MsgId, msgbuf: &mut MsgBuf) -> Result<(), Box<dyn std::error::Error>> {
    let flags = 0;
    ipc::msgrcv(id, msgbuf, flags)?;
    Ok(())
}
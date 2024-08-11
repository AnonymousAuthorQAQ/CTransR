use std::io::{self, Read};
use std::process::wait;
use std::ptr::null;
use std::ffi::CString;
use ipc::{ipc_private, ipc_mode, ipc_nowait, msgget, msgrcv, msgsnd, msgbuf, pid_t};

const MY_TYPE: i32 = 9527;

fn main() {
    let msgid = msgget(ipc_private(), ipc_mode!(0666 | ipc_excl));
    
    if msgid == -1 {
        eprintln!("msgget failed");
        return;
    }

    match fork() {
        Ok(pid) if pid > 0 => {
            std::thread::sleep(std::time::Duration::from_secs(1));
            
            let mut buff = msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            print!("Please enter a string you want to send:\n");
            io::stdin().read_line(&mut buff.mtext).expect("Failed to read line");
            
            print!("Please enter a number you want to send:\n");
            io::stdin().read_line(&mut buff.number).expect("Failed to read number");

            if msgsnd(msgid, &buff, (std::mem::size_of::<msgbuf>() - std::mem::size_of::<i32>()) as i32, ipc_nowait) == -1 {
                eprintln!("msgsnd failed");
            }

            wait();
        }
        Ok(0) => {
            let mut buff = msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            if msgrcv(msgid, &mut buff, (std::mem::size_of::<msgbuf>() - std::mem::size_of::<i32>()) as i32, MY_TYPE, ipc_nowait) == -1 {
                eprintln!("msgrcv failed");
            }

            println!("Child process read from msg: {}", buff.mtext, buff.number);
        }
        Err(e) => {
            eprintln!("fork failed: {}", e);
        }
    }
}
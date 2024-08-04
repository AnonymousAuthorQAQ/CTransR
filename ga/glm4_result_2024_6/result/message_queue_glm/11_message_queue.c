use std::io::{self, Read};
use std::process::exit;
use nix::sys::ipc::{key_t, msgget, ipc_perm};
use nix::sys::msg::{MsgFlag, Msgsnd, Msgrcv};
use nix::unistd::{fork, waitpid, Pid};
use nix::sys::wait::WaitStatus;

const MY_TYPE: i32 = 9527;

#[derive(Debug)]
struct MsgBuf {
    mtype: i32,
    mtext: [i8; 100],
    number: i32,
}

fn main() {
    let key = key_t::from_raw(0);
    let mut perms = ipc_perm::default();
    perms.mode = 0o666;

    let msgid = msgget(key, MsgFlag::CREAT | MsgFlag::EXCL, &perms).expect("msgget failed");

    match fork() {
        Ok(0) => {
            // Child process
            let mut msgbuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            let _ = io::stdin().read(&mut msgbuf.mtext);
            let _ = io::stdin().read_line(&mut String::new()); // Consume newline
            let _ = io::stdin().read_line(&mut String::new()); // Read the number

            let _ = Msgrcv::msgrcv(msgid, &msgbuf, MsgFlag::empty()).expect("msgrcv failed");
            println!("Child process read from msg: {}", msgbuf.mtext.iter().take_while(|&c| c != 0).collect::<String>(), msgbuf.number);
        }
        Ok(pid) => {
            // Parent process
            let mut msgbuf = MsgBuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            print!("Please enter a string you want to send:\n");
            io::stdout().flush().expect("flush failed");
            let _ = io::stdin().read(&mut msgbuf.mtext);
            print!("Please enter a number you want to send:\n");
            io::stdout().flush().expect("flush failed");
            let _ = io::stdin().read_line(&mut String::new()); // Consume newline
            let _ = io::stdin().read_line(&mut String::new()); // Read the number

            let _ = Msgsnd::msgsnd(msgid, &msgbuf, MsgFlag::empty()).expect("msgsnd failed");

            match waitpid(Pid::from_raw(pid), None) {
                Ok(WaitStatus::Exited(_)) => {
                    println!("Child process exited");
                }
                Ok(WaitStatus::Signaled(_)) => {
                    println!("Child process was terminated by a signal");
                }
                Ok(WaitStatus::Stopped(_)) => {
                    println!("Child process was stopped by a signal");
                }
                Ok(WaitStatus::Continued) => {
                    println!("Child process was continued");
                }
                Err(e) => {
                    println!("Error waiting for child: {}", e);
                }
            }
        }
        Err(e) => {
            println!("fork failed: {}", e);
            exit(1);
        }
    }
}
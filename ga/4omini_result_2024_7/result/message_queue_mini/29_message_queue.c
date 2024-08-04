use libc::{msgget, msgsnd, msgrcv, IPC_PRIVATE, key_t};
use std::ffi::{CString, CStr};
use std::io::{self, Write};
use std::mem;
use std::os::raw::{c_void, c_long};
use std::ptr;
use std::process::{exit, Command};
use std::thread;
use std::time::Duration;

const MY_TYPE: c_long = 9527;

#[repr(C)]
struct Msgbuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    unsafe {
        let msgid = msgget(IPC_PRIVATE, 0o666 | libc::IPC_EXCL);
        if msgid == -1 {
            eprintln!("msgget error");
            exit(1);
        }

        let pid = Command::new("sh") // replace it later with a safer abstraction for process creation
            .arg("-c")
            .arg("fork")
            .spawn()
            .expect("Failed to start child process");
            
        if pid.id() > 0 {
            // Parent process
            thread::sleep(Duration::from_secs(1));

            let mut buff: Msgbuf = Msgbuf {
                mtype: MY_TYPE,
                mtext: [0; 100],
                number: 0,
            };

            // Prompt & read user input for message
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buff.mtext).unwrap();

            let mut input = String::new();
            print!("Please enter a number you want to send: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            buff.number = input.trim().parse::<i32>().unwrap();

            msgsnd(msgid, &buff as *const _ as *const c_void, mem::size_of::<Msgbuf>() as libc::size_t, 0);
            
            // Wait for the child to finish
            match pid.wait() {
                Ok(_) => (),
                Err(err) => eprintln!("Child process encountered an error: {}", err),
            }
        } else {
            // Child process
            let mut buff: Msgbuf = Msgbuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            msgrcv(msgid, &mut buff as *mut _ as *mut c_void, mem::size_of::<Msgbuf>() as libc::size_t - mem::size_of::<c_long>() as libc::size_t, MY_TYPE, 0);
            // Create a null-terminated string from mtext
            let text = CStr::from_ptr(buff.mtext.as_ptr() as *const _);
            println!(
                "Child process read from msg: {}, {}",
                text.to_string_lossy(),
                buff.number
            );
        }
    }
}
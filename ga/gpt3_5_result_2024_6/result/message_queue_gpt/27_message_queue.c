use std::process::Command;
use std::os::unix::process::CommandExt;
use std::ptr;
use std::ffi::CString;
use std::mem;
use std::io::{self, Write};
use libc::{msgget, msgctl, msgsnd, msgrcv, IPC_PRIVATE, IPC_EXCL, msgbuf, c_void, c_long, c_int, msqid_ds, MSG_INFO};

const MY_TYPE: c_long = 9527;

fn main() {
    let mut msgid: c_int;
    let mut pid: c_int;

    unsafe {
        msgid = msgget(IPC_PRIVATE, 0o666 | IPC_EXCL);
        if msgid == -1 {
            panic!("msgget failed");
        }
    }

    match unsafe { fork() } {
        Ok(child_pid) => {
            if child_pid > 0 {
                std::thread::sleep(std::time::Duration::from_secs(1));

                let mut buff: msgbuf = msgbuf {
                    mtype: MY_TYPE,
                    mtext: [0; 100],
                    number: 0,
                };

                print!("Please enter a string you want to send: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.pop(); // Remove newline character

                let input_cstr = CString::new(input).unwrap();
                unsafe {
                    ptr::copy_nonoverlapping(input_cstr.as_ptr(), buff.mtext.as_mut_ptr() as *mut c_void, input_cstr.as_bytes().len());
                }

                print!("Please enter a number you want to send: ");
                io::stdout().flush().unwrap();
                let mut number_input = String::new();
                io::stdin().read_line(&mut number_input).unwrap();
                let number: i32 = number_input.trim().parse().unwrap();
                buff.number = number;

                unsafe {
                    msgsnd(msgid, &buff as *const _ as *const c_void, mem::size_of_val(&buff) - mem::size_of::<c_long>(), 0);
                }

                Command::new("waitpid")
                    .arg(format!("{}", child_pid))
                    .spawn()
                    .expect("waitpid failed");
            } else if child_pid == 0 {
                let mut buff: msgbuf = msgbuf {
                    mtype: 0,
                    mtext: [0; 100],
                    number: 0,
                };

                unsafe {
                    msgrcv(msgid, &mut buff as *mut _ as *mut c_void, mem::size_of_val(&buff) - mem::size_of::<c_long>(), MY_TYPE, 0);
                }

                let received_string = CString::from_vec_unchecked(buff.mtext.iter().map(|&c| c as u8).collect());
                let received_str = received_string.to_str().unwrap();

                println!("Child process read from msg: {}, {}", received_str, buff.number);
            }
        },
        Err(_) => panic!("fork failed"),
    }
}
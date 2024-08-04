use libc::{key_t, IPC_EXCL, IPC_PRIVATE, msgbuf, msgget, msgrcv, msgsnd, rand, getpid};
use std::ffi::CString;
use std::mem::size_of;
use std::process::{Command, Child};
use std::ptr::{null_mut};
use std::os::raw::{c_int, c_long, c_short};

const MY_TYPE: c_long = 9527;

fn main() {
    let msgid: c_int;
    let mut pid: c_int;

	let mut buffer = msgbuf {
		mtype: 0,
		mtext: [0; 100],
		number: 0,
	};

    unsafe {
        msgid = msgget(IPC_PRIVATE as key_t, 0o666 | IPC_EXCL as i32);

        if msgid == -1 {
            perror("msgget");
            panic!("Could not create message queue");
        }

        pid = fork();
    }

    match pid {
        0 => {
            unsafe {
                msgrcv(msgid, &mut buffer as *mut _ as *mut std::os::raw::c_void, 
                    size_of::<msgbuf>() - size_of::<c_long>(), MY_TYPE, 0);
                
                let message = CString::new(buffer.mtext.iter().map(|&c| c as u8).take_while(|&c| c != 0)
                    .collect::<Vec<u8>>()).expect("Failed to parse CString");

                println!("Child process read from msg: {}, {}", message.to_str().unwrap(), buffer.number);
            }
        }
        _ if pid > 0 => {
            std::thread::sleep(std::time::Duration::from_secs(1));

            println!("Please enter a string you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            let input_num: i32 = rand::random();

        	unsafe {
        		buffer.mtype = MY_TYPE;
        		buffer.number = input_num;
                input.truncate(99); // Removing newline character
                input.bytes().take(100).enumerate().for_each(|(i, byte)| {
                    buffer.mtext[i] = byte as c_short;
                });

        		msgsnd(msgid, &buffer as *const _ as *mut std::os::raw::c_void, size_of::<msgbuf>() - size_of::<c_long>(), 0); 
            }

            wait(pid);
        }
        _ => {
            panic!("Fork failed");
        }
    }
}

unsafe fn fork() -> c_int {
    libc::fork()
}

unsafe fn wait(pid: c_int) {
    libc::waitpid(pid, null_mut(), 0);
}

fn perror(msg: &str) {
    unsafe {
        libc::perror(std::ffi::CString::new(msg).unwrap().as_ptr());
    }
}
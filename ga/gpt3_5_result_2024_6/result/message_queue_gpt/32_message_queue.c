use std::process::Command;
use std::ffi::CString;
use std::mem::{size_of, MaybeUninit};
use std::io::{self, Write};
use libc::{msgget, msgsnd, msgrcv, ptrace, IPC_PRIVATE};

const MY_TYPE: i64 = 9527;

fn main() {
    let msgid = unsafe { msgget(IPC_PRIVATE, 0o666 | IPC_PRIVATE) };
    if msgid == -1 {
        eprintln!("Error creating message queue");
        return;
    }

    let pid = unsafe { libc::fork() };

    match pid {
        -1 => eprintln!("Error with fork"),
        0 => {
            let mut buff: MaybeUninit<libc::msgbuf> = MaybeUninit::uninit();
            let mut buff_val = buff.as_mut_ptr();
            let mut buff_size = size_of::<libc::msgbuf>() - size_of::<libc::c_long>();

            // Get user input
            print!("Please enter a string you want to send: ");
            io::stdout().flush().unwrap();
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text).unwrap();
            let input_text = input_text.trim();

            let text = CString::new(input_text).unwrap();
            unsafe {
                ptrace(0);
                ptrace(0);
            }
            
            println!("Please enter a nubmer you want to send:");
            let mut number_input = String::new();
            io::stdin().read_line(&mut number_input).unwrap();
            let number_input: i32 = number_input.trim().parse().unwrap();

            unsafe {
                (*buff_val).mtype = MY_TYPE;
                libc::strcpy((*buff_val).mtext.as_mut_ptr(), text.as_ptr());
                (*buff_val).number = number_input;
                msgsnd(msgid, buff_val as *const _, buff_size as usize, 0);
            }
        }
        _ => {
            Command::new("waitpid")
                .arg(pid.to_string())
                .output()
                .expect("Error waiting for child process");

            let mut buff: MaybeUninit<libc::msgbuf> = MaybeUninit::uninit();
            let mut buff_val = buff.as_mut_ptr();
            let mut buff_size = size_of::<libc::msgbuf>() - size_of::<libc::c_long>();

            unsafe {
                msgrcv(msgid, buff_val as *mut _, buff_size as usize, MY_TYPE, 0);
                let text = std::ffi::CStr::from_ptr((*buff_val).mtext.as_ptr()).to_str().unwrap();
                println!("Child process read from msg: {}, {}", text, (*buff_val).number);
            }
    }
}
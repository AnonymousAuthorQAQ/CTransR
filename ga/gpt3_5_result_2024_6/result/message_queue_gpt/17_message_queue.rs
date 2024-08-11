use std::process;
use std::ptr;
use libc;

const MY_TYPE: i64 = 9527;

fn main() {
    let key = unsafe { libc::IPC_PRIVATE };
    let msgid = unsafe { libc::msgget(key, libc::IPC_CREAT | 0o666) };

    if msgid == -1 {
        eprintln!("Error in msgget");
        return;
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        unsafe {
            libc::sleep(1);

            let mut buff: libc::msgbuf = std::mem::zeroed();
            buff.mtype = MY_TYPE;

            println!("Please enter a string you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            ptr::copy_nonoverlapping(input.as_ptr(), buff.mtext.as_mut_ptr(), input.len().min(buff.mtext.len()));
            
            println!("Please enter a number you want to send:");
            let mut num_input = String::new();
            std::io::stdin().read_line(&mut num_input).unwrap();
            let number: i32 = num_input.trim().parse().unwrap();
 			buff.number = number;

            libc::msgsnd(msgid, &buff as *const _ as *const libc::c_void, std::mem::size_of_val(&buff) - std::mem::size_of::<i64>(), 0);
        }

        process::wait().unwrap();
    } else if pid == 0 {
        // Child process
        let mut buff: libc::msgbuf = std::mem::zeroed();
        unsafe {
            libc::msgrcv(msgid, &mut buff as *mut _ as *mut libc::c_void, 
                          std::mem::size_of_val(&buff) - std::mem::size_of::<i64>(), MY_TYPE, 0);

            let bytes = std::slice::from_raw_parts(buff.mtext.as_ptr(), buff.mtext.iter().position(|&x| x == 0).unwrap());
            let text = std::str::from_utf8(bytes).unwrap();
           
            println!("Child process read from msg: {}, {}", text, buff.number);
        }
    } else {
        eprintln!("Fork error");
    }
}
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::ffi::CString;
use std::ptr;
use std::mem;
 
fn main() {
    let key = 9527;
    let ftok_path = CString::new(".").unwrap();
    let project_id = key as i32;
    let msgid = unsafe {
        libc::msgget(libc::ftok(ftok_path.as_ptr(), project_id), 0o666 | libc::IPC_CREAT)
    };
 
    if msgid == -1 {
        panic!("Error creating message queue");
    }
 
    let mut msgbuf: Vec<u8> = vec![0; mem::size_of::<libc::msgbuf>()];
    let mtype: i64 = key as i64;
 
    let pid = unsafe { libc::fork() };
    match pid {
        -1 => panic!("Fork failed"),
        0 => {
            let mut msgbuf_c: libc::msgbuf = libc::msgbuf {
                mtype: mtype,
                mtext: [0; 100],
                number: 0
            };
            let mut input_string = String::new();
            println!("Please enter a string you want to send:");
            std::io::stdin().read_line(&mut input_string).unwrap();
            input_string.pop(); // remove newline
            
            let input_number: i32 = loop {
                let mut input = String::new();
                println!("Please enter a number you want to send:");
                std::io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse() {
                    Ok(num) => break num,
                    Err(_) => continue,
                }
            };
 
            let input_bytes = input_string.as_bytes();
            input_bytes.copy_to_slice(&mut msgbuf_c.mtext[0..input_bytes.len()]);
            msgbuf_c.number = input_number;
 
            msgbuf.copy_from_slice(unsafe {
                std::slice::from_raw_parts(&msgbuf_c as *const _ as *const u8, mem::size_of_val(&msgbuf_c))
            });
            unsafe {
                libc::msgsnd(msgid, msgbuf.as_ptr() as *const libc::c_void, msgbuf.len() as usize, 0);
            }
        },
        _ => {
            unsafe {
                libc::wait(std::ptr::null_mut());
            }
            let mut received_msg: libc::msgbuf = mem::zeroed();
            unsafe {
                libc::msgrcv(msgid, &mut received_msg as *mut _ as *mut libc::c_void, msgbuf.len() as usize, mtype, 0);
            }
            let c_string = std::ffi::CStr::from_ptr(&received_msg.mtext[0] as *const _);
            let string = c_string.to_str().unwrap();
            println!("Child process read from msg: {}, {}", string, received_msg.number);
        },
    }
}
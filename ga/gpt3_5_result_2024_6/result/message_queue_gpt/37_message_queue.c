use libc::{ftok, shmget, shmctl, shmat, shmdt, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, key_t, c_void, c_int, c_char, msgget, msgctl, msgbuf, msgrcv, msgsnd};
use std::ffi::CString;
use std::{mem, ptr};
use std::u32;
use std::process::Command;
use std::io;
use std::io::Write;

const MY_TYPE: i64 = 9527;
const IPC_RMID: i32 = 0;

struct MsgBuf {
    mtype: i64,
    mtext: [c_char; 100],
    number: i32,
}

fn main() {
    let msgkey: key_t = unsafe { ftok(CString::new(".").unwrap().as_ptr(), 65)};
    let mut msgid = unsafe { msgget(msgkey, IPC_CREAT | IPC_EXCL | 0o666) };

    if msgid == -1 {
        panic!("msgget error");
    }

    let mut pid = unsafe { Command::new("sh").spawn().unwrap().id() };

    if pid > 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        let mut buff: MsgBuf = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100], 
            number: 0,
        };

        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();
        buff.mtext = text.trim_end().chars().map(|c| c as _).collect::<Vec<c_char>>().try_into().unwrap();

        print!("Please enter a number you want to send: ");
        io::stdout().flush().unwrap();
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        buff.number = num.trim().parse().expect("Please enter a valid number");

        let res = unsafe { msgsnd(msgid, &buff as *const MsgBuf as *const c_void, mem::size_of::<MsgBuf>() as c_int - mem::size_of::<i64>() as i64, 0) };
        
        let _ = Command::new("wait").arg("-pid").arg(pid.to_string()).output().ok();

        if res == -1 {
            panic!("msgsend error");
        }
    }
    else if pid == 0 {
        println!("Child process is waiting for msg:");
        
        let mut buff: MsgBuf = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        let _ = unsafe { msgrcv(msgid, &mut buff as *mut MsgBuf as *mut c_void, mem::size_of::<MsgBuf>() as i64 - mem::size_of::<i64>() as i64, MY_TYPE, 0) };

        let received_text = buff.mtext.into_iter().map(|c| c as u8).collect::<Vec<u8>>();
        let received_text = String::from_utf8(received_text).unwrap();

        println!("Child process read from msg: {}, {}", received_text, buff.number);
    }
    else {
        panic!("fork error");
    }

    let res = unsafe { msgctl(msgid, IPC_RMID, ptr::null_mut()) };
    if res == -1 {
        panic!("msgctl error");
    }
}
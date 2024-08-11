use std::ffi::CString;
use std::os::raw::{c_long, c_void};
use std::ptr;
use std::process;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    unsafe {
        let key = libc::IPC_PRIVATE;
        let msgid = libc::msgget(key, 0o666 | libc::IPC_EXCL);

        if msgid == -1 {
            eprintln!("msgget error");
            return;
        }

        let pid = libc::fork();

        if pid > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            let mut buff = MsgBuf {
                mtype: 9527,
                mtext: [0; 100],
                number: 0,
            };

            // println!("Please enter a string you want to send:");
            std::io::stdin().read_line(&mut buff.mtext).unwrap();
            // println!("Please enter a number you want to send:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let number: i32 = input.trim().parse().unwrap();
            buff.number = number;

            libc::msgsnd(msgid, &buff as *const MsgBuf as *const c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>() as i64, 0);

            libc::waitpid(pid, ptr::null_mut(), 0);
        } else if pid == 0 {
            // println!("Child process is waiting for msg:");
            let mut buff = MsgBuf {
                mtype: 0,
                mtext: [0; 100],
                number: 0,
            };

            libc::msgrcv(msgid, &mut buff as *mut MsgBuf as *mut c_void, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<c_long>() as i64, 9527, 0);
            
            let received_str = CString::from_vec_unchecked(buff.mtext.to_vec()).to_str().unwrap();
            println!("Child process read from msg: {}, {}", received_str, buff.number);
        } else {
            eprintln!("fork error");
        }
    }
}
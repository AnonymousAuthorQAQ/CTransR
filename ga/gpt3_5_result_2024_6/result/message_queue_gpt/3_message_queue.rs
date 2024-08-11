use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;
use std::mem;
use std::process;

#[repr(C)]
struct MsgBuf {
    mtype: i64,
    mtext: [u8; 100],
    number: c_int,
}

fn main() {
    let key = ftok("/tmp", 1);
    let msgid = msgget(key, 0o666);

    if msgid == -1 {
        panic!("msgget failed");
    }

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        unsafe {
            libc::sleep(1);
        }

        let mut buf: MsgBuf = unsafe { mem::zeroed() };
        buf.mtype = 9527;

        println!("Please enter a string you want to send:");
        let mut input_text = String::new();
        std::io::stdin().read_line(&mut input_text).unwrap();
        let input_text = CString::new(input_text.trim()).unwrap();
        let input_text_bytes = input_text.as_bytes();
        buf.mtext[..input_text_bytes.len()].copy_from_slice(input_text_bytes);

        println!("Please enter a number you want to send:");
        let mut input_number = String::new();
        std::io::stdin().read_line(&mut input_number).unwrap();
        let input_number: c_int = input_number.trim().parse().unwrap();
        buf.number = input_number;

        msgsnd(msgid, &mut buf as *mut MsgBuf, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 0);

        unsafe {
            libc::waitpid(pid, ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        let mut buf: MsgBuf = unsafe { mem::zeroed() };
        println!("Child process is waiting for msg:");

        msgrcv(msgid, &mut buf as *mut MsgBuf, std::mem::size_of::<MsgBuf>() - std::mem::size_of::<i64>(), 9527, 0);

        let text = String::from_utf8_lossy(&buf.mtext);
        println!("Child process read from msg: {}, {}", text, buf.number);
    } else {
        panic!("fork failed");
    }
}

fn ftok(path: &str, id: i32) -> i64 {
    let ftok_path = CString::new(path).unwrap();
    let ftok_proj_id = id as i32;
    unsafe { libc::ftok(ftok_path.as_ptr(), ftok_proj_id) as i64 }
}

fn msgget(key: i64, msgflg: i32) -> i32 {
    unsafe { libc::msgget(key, msgflg) }
}

fn msgsnd(msqid: i32, msgp: *mut MsgBuf, msgsz: usize, msgflg: i32) -> i32 {
    unsafe { libc::msgsnd(msqid, msgp as *const _, msgsz, msgflg) }
}

fn msgrcv(msqid: i32, msgp: *mut MsgBuf, msgsz: usize, msgtyp: i64, msgflg: i32) -> i32 {
    unsafe { libc::msgrcv(msqid, msgp as *mut _, msgsz, msgtyp, msgflg) }
}
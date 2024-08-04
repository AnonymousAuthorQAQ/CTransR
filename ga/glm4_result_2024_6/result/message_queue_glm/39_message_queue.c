use std::io::{self, Read};
use std::os::unix::ipc::MsgQue;
use std::process::exit;
use std::ptr;
use std::ffi::CString;

fn main() {
    let msgid = match MsgQue::new(0, 0o666 | 0o200) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("msgget failed: {}", e);
            exit(1);
        }
    };

    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent
        unsafe {
            libc::sleep(1);
        }

        let mut buff = MsgQue::msgbuf {
            mtype: 9527,
            mtext: [0; 100],
            number: 0,
        };

        print!("Please enter a string you want to send:\n");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = CString::new(input).unwrap();
        ptr::copy(input.as_ptr(), buff.mtext.as_mut_ptr(), input.as_bytes_with_nul().len());

        print!("Please enter a number you want to send:\n");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        buff.number = input.trim().parse().unwrap();

        msgid.send(&buff, MsgQue::MSG_NOSIGNAL).unwrap();
        unsafe {
            libc::waitpid(pid, ptr::null_mut(), 0);
        }
    } else if pid == 0 {
        // Child
        let mut buff = MsgQue::msgbuf {
            mtype: 9527,
            mtext: [0; 100],
            number: 0,
        };

        msgid.recv(&mut buff, MsgQue::MSG_NOSIGNAL).unwrap();
        println!("Child process read from msg: {}", buff.mtext.to_string_lossy(), buff.number);
    } else {
        eprintln!("fork failed");
        exit(1);
    }
}
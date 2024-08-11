use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::ptr;
use std::ffi::CString;
use std::mem;

const MY_TYPE: u32 = 9527;

#[repr(C)]
#[derive(Debug)]
struct MsgBuf {
    mtype: u32,
    mtext: [u8; 100],
    number: i32,
}

fn main() -> io::Result<()> {
    let (parent, child) = os_pipe::pipe()?;
    
    let pid = unsafe { libc::fork() };

    if pid > 0 {
        // Parent process
        let mut msg_buf = MsgBuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };

        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut msg_buf.mtext.iter_mut().take(99).collect::<String>())?;
        
        println!("Please enter a number you want to send:");
        io::stdin().read_line(&mut msg_buf.number.to_string())?;
        
        let msg_size = mem::size_of::<MsgBuf>() as isize;

        // Write the message to the child
        let _ = child.write_all(unsafe { &*(msg_buf as *const MsgBuf as *const [u8; 104]) })?;
        
        // Wait for the child process to finish
        let _ = unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
    } else if pid == 0 {
        // Child process
        let mut msg_buf = MsgBuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        // Read the message from the parent
        let _ = parent.read_exact(unsafe { &mut *(msg_buf as *mut MsgBuf as *mut [u8; 104]) })?;
        
        // Convert the message to a String
        let msg_text = String::from_utf8_lossy(&msg_buf.mtext);
        println!("Child process read from msg: {}, {}", msg_text.trim(), msg_buf.number);
    } else {
        // Fork failed
        eprintln!("Fork failed");
    }

    Ok(())
}
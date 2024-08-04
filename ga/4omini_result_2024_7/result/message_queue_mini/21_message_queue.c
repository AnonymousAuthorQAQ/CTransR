use nix::unistd::{fork, ForkType, Pid};
use nix::sys::msg::*;
use nix::errno::Errno;
use std::io::{self, Write};
use std::process::exit;
use std::mem;

const MY_TYPE: usize = 9527;

fn main() {
    let msgid = msgget(IPC_PRIVATE, 0o666 | IPC_CREAT)
        .expect("Failed to create message queue");

    let pid = fork().expect("Failed to fork process");

    if pid.is_parent() {
        // Parent process
        let mut buff: MsgBuf = MsgBuf {
            mtype: MY_TYPE as u64,
            mtext: [0; 100],
        };

        let mut input = String::new();
        print!("Please enter a string you want to send:\n");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        buff.mtext[..input.trim().len()].copy_from_slice(input.trim().as_bytes());

        print!("Please enter a number you want to send:\n");
        io::stdout().flush().unwrap();
        let mut number_input = String::new();
        io::stdin().read_line(&mut number_input).unwrap();
        let number: i32 = number_input.trim().parse().expect("Invalid number");

        // Send message (number is sent in a separate buffer)
        let number_buf = number.to_le_bytes(); // convert to bytes for sending
        let number_size = mem::size_of::<i32>();
        let msg_size = mem::size_of::<MsgBuf>() - mem::size_of::<u64>();

        // First, send the message text
        msgsnd(msgid, &buff, msg_size).expect("Failed to send message text");

        // Now send the number in a separate message
        msgsnd(msgid, &MsgBuf {
            mtype: MY_TYPE as u64,
            mtext: number_buf,
        }, number_size).expect("Failed to send number");

        // Wait for child process
        pid.wait().expect("Failed to wait for child");
    } else {
        // Child process
        let mut buff: MsgBuf = MsgBuf {
            mtype: MY_TYPE as u64,
            mtext: [0; 100],
        };

        // Receive message text
        let msg_size = mem::size_of::<MsgBuf>() - mem::size_of::<u64>();
        msgrcv(msgid, &mut buff, msg_size, MY_TYPE as u64, 0).expect("Failed to receive message");

        let number: [u8; 4] = [0; 4];
        msgrcv(msgid, &mut MsgBuf { mtype: MY_TYPE as u64, mtext: number }, mem::size_of::<i32>(), MY_TYPE as u64, 0).expect("Failed to receive number");

        let number = i32::from_le_bytes(number);
        println!("Child process read from msg: {}, {}", String::from_utf8_lossy(&buff.mtext), number);
    }
}
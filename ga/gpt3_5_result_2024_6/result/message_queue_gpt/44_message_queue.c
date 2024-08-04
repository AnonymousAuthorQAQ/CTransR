use libc::{fork, getpid, scanf, sleep, waitpid};
use libc::{msgrcv, msgsnd, IPC_EXCL, IPC_PRIVATE};

const MY_TYPE: i64 = 9527;

struct msgbuf {
    mtype: i64,
    mtext: [u8; 100],
    number: i32,
}

fn main() {
    let msgid = unsafe { msgsnd(IPC_PRIVATE, 0o666 | IPC_EXCL) };

    if msgid == -1 {
        eprintln!("Error in msgget");
        return;
    }

    let pid = unsafe { fork() };

    if pid > 0 {
        unsafe { sleep(1) };

        let mut buff = msgbuf {
            mtype: MY_TYPE,
            mtext: [0; 100],
            number: 0,
        };
        
        let input: String;
        unsafe {         
            println!("Please enter a string you want to send:");
           	input = String::new();
           	scan("%s".as_ptr(), &input); 

            println!("Please enter a nubmer you want to send:");
            scanf("%d".as_ptr(), &mut buff.number);
        }

        unsafe { msgsnd(msgid, &buff, 116, 0) };

        unsafe { waitpid(pid, std::ptr::null_mut(), 0) };
    } else if pid == 0 {
        let mut buff = msgbuf {
            mtype: 0,
            mtext: [0; 100],
            number: 0,
        };

        unsafe {
            println!("Child process is waiting for msg:");
            msgrcv(msgid, &mut buff, 116, MY_TYPE, 0);
            println!("Child process read from msg: {}, {}", buff.mtext, buff.number);
        }
    } else {
        eprintln!("Error in fork");
    }
}
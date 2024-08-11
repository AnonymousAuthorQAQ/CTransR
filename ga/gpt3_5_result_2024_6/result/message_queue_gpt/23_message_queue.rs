use libc;
use std::mem;
use std::ptr;

const MY_TYPE: i64 = 9527;

fn main() {
    let msgid: i32;
    let mut pid: i32;

    unsafe {
        let mut msgbuf: libc::msqid_ds = mem::zeroed();
        let mut msgrcv_buff: [libc::c_char; 100] = [0; 100];

        msgid = libc::msgget(libc::IPC_PRIVATE, libc::IPC_CREAT | 0o666);

        if msgid == -1 {
            perror("msgget");
            return;
        }

        pid = libc::fork();

        if pid > 0 {
            libc::sleep(1);

            let mut buff: libc::msgbuf = libc::msgbuf {
                mtype: MY_TYPE,
                mtext: msgrcv_buff,
                number: 0,
            };

            println!("Please enter a string you want to send:");
            let mut input_string = String::new();
            std::io::stdin().read_line(&mut input_string).unwrap();
            input_string.pop(); // Remove trailing newline

            libc::strcpy(buff.mtext.as_mut_ptr(), input_string.as_ptr() as *const i8);

            println!("Please enter a number you want to send:");
            let mut input_number = String::new();
            std::io::stdin().read_line(&mut input_number).unwrap();
            let number: i32 = input_number.trim().parse().unwrap();
            buff.number = number;

            libc::msgsnd(msgid, &buff as *const _ as *mut libc::c_void, std::mem::size_of_val(&buff) -  std::mem::size_of_val(&buff.mtype) as usize, 0);

            libc::waitpid(pid, std::ptr::null_mut(), 0);
        } else if pid == 0 {
            println!("Child process is waiting for msg:");
            libc::msgrcv(msgid, &mut msgbuf as *mut libc::msqid_ds as *mut libc::c_void, std::mem::size_of_val(&msgbuf) as usize - std::mem::size_of_val(&msgbuf.msg_perm) as usize, MY_TYPE, 0);
            let buff: *mut libc::msgbuf = msgbuf.msg_first!;
            let number = (*buff).number;
            let mtext = std::ffi::CStr::from_ptr((*buff).mtext.as_ptr()).to_str().unwrap();
            println!("Child process read from msg: {}, {}", mtext, number);
        } else {
            perror("fork");
        }
    }
}

fn perror(msg: &str) {
    unsafe {
        libc::perror(msg.as_ptr() as *const libc::c_char);
    }
}
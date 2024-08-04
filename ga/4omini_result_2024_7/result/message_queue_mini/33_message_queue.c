use nix::msg::Msg;
use nix::unistd::{fork, ForkTag, sleep, Fork};
use nix::sys::msg::{self, MsgQueue};
use std::io::{self, Write};

const MY_TYPE: u64 = 9527;

fn main() {
    let key = msg::ftok("/tmp", 0).unwrap(); // Create a key using a pathname (used for shm).
    
    let msgid = MsgQueue::new(key, 0o666).unwrap(); // Create message queue with permissions

    let pid = unsafe { fork().unwrap() }; // Fork

    if let Ok(ForkTag::Parent { child: _ }) = pid {
        let mut buff = Msg::new(MyMsg { mtype: MY_TYPE, mtext: String::new(), number: 0 }).unwrap(); // Prepare our buffer

        sleep(1);

        let mut input_string = String::new();
        print!("Please enter a string you want to send: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_string).unwrap();         
        buff.data().mtext = input_string.trim().to_string(); // Trim newline
       
        println!("Please enter a number you want to send: ");
        let mut number_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut number_input).unwrap();

        buff.data().number = number_input.trim().parse::<i32>().unwrap();
        
        unsafe { msgid.send(&buff).unwrap() }; // Send the message

    } else if let Ok(ForkTag::Child) = pid {
        // In this child process, receive message
        let mut received_message = msg::Msg::new_dense_delivered_data();
        unsafe { msgid.receive(&mut received_message).unwrap() }; // Read the message
        println!("Child process read from msg: {}, {}", 
                 received_message.data().mtext,
                 received_message.data().number);
    } else {
        println!("Fork failed");
    }

    // Raise Message Queue to work with focus: release resources / clean up proc resource
    unsafe { MsgQueue::destroy(msgid).unwrap() }; // Cleanup when we're done
}
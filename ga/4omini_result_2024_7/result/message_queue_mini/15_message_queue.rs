use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() -> io::Result<()> {
    // Create a Mutex to be safely shared
    let shared_data = Arc::new(Mutex::new((String::new(), 0)));

    // Clone the Arc for use in the child process
    let shared_data_clone = Arc::clone(&shared_data);

    // Spawn a child process
    let child = thread::spawn(move || {
        // Set up for reading input from the parent
        let (input_str, number) = {
            // Lock and borrow the mutex
            let data = shared_data_clone.lock().unwrap();
            let input_str = data.0.clone();
            let number = data.1;
            (input_str, number)
        };

        // The output of the "child process"
        println!("Child process read: {}, {}", input_str, number);
    });

    // Simulating the parent process waits a little before sending data
    thread::sleep(std::time::Duration::from_secs(1));

    // Get input for the shared data
    let mut buffer = String::new();
    print!("Please enter a string you want to send: ");
    io::stdout().flush().unwrap(); // ensure prompt is shown
    io::stdin().read_line(&mut buffer).unwrap();
    let trimmed_string = buffer.trim().to_string();

    // Get an integer input
    let mut number = String::new();
    print!("Please enter a number you want to send: ");
    io::stdout().flush().unwrap(); // ensure prompt is shown
    io::stdin().read_line(&mut number).unwrap();
    let number: i32 = number.trim().parse().unwrap();

    {
        let mut data = shared_data.lock().unwrap();
        data.0 = trimmed_string; // Store string in the Mutex
        data.1 = number; // Store number in the Mutex
    }

    // Wait for the child thread to finish
    child.join().unwrap();

    Ok(())
}
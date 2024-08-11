use std::alloc::{GlobalAlloc, Layout};
use std::io::{self, Write};

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec as dynamic array in Rust
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, String> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: vec![0; max_elements],
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full Queue".to_string());
        }

        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity; // Circular increment
        self.array[self.rear] = x;

        Ok(())
    }

    fn front(&self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        Ok(self.array[self.front])
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        self.size -= 1;
        self.front = (self.front + 1) % self.capacity; // Circular increment

        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        let value = self.front()?;
        self.dequeue()?;
        Ok(value)
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }
}

fn main() {
    let mut queue_length = String::new();
    print!("Enter queue length (minimum is {}): ", MIN_QUEUE_SIZE);
    io::stdout().flush().unwrap(); // Ensure prompt is shown before input

    io::stdin().read_line(&mut queue_length).unwrap();
    let queue_length: usize = match queue_length.trim().parse() {
        Ok(num) if num >= MIN_QUEUE_SIZE => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = match Queue::new(queue_length) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        // Dequeue example, which shouldn't happen here since it's empty
        let _ = queue.dequeue().unwrap(); // Just for code completion
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            let _ = queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let mut enqueue_value = String::new();
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap(); // Ensure prompt is shown
        io::stdin().read_line(&mut enqueue_value).unwrap();
        if let Ok(value) = enqueue_value.trim().parse::<i32>() {
            let _ = queue.enqueue(value); 
        } else {
            eprintln!("Invalid input");
            return;
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => eprintln!("{}", e),
        }
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        let _ = queue.enqueue(i as i32);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front().unwrap());
        let _ = queue.dequeue(); 
    }
}
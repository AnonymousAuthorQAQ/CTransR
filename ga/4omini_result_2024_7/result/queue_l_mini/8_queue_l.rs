use std::io;
use std::ptr;
use std::mem;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: Vec<i32>, // Using Vec to automatically handle memory
}

impl Queue {
    pub fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: -1,
            size: 0,
            array: vec![0; max_elements],
        })
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity as isize;
            self.array[self.rear as usize] = x;
            Ok(())
        }
    }

    pub fn front(&self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        Ok(self.array[self.front])
    }

    pub fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            Ok(())
        }
    }

    pub fn front_and_dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        } else {
            let front_value = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            Ok(front_value)
        }
    }

    pub fn dispose(self) {
        // Rust automatically reclaims the memory when the Queue struct goes out of scope
    }
}

fn main() {
    let mut queue_length = String::new();
    io::stdin()
        .read_line(&mut queue_length)
        .expect("Failed to read line");

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
    if !queue.is_empty() {
        if let Err(e) = queue.dequeue() {
            println!("{}", e);
        }
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i).expect("Failed to enqueue");
        }
    }

    // Test: Attempt to enqueue to a full queue
    let mut enqueue_value = String::new();
    io::stdin()
        .read_line(&mut enqueue_value)
        .expect("Failed to read line");

    if !queue.is_full() {
        let value: i32 = match enqueue_value.trim().parse() {
            Ok(num) => num,
            _ => {
                eprintln!("Invalid input");
                return;
            }
        };
        queue.enqueue(value).expect("Failed to enqueue");
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => println!("{}", e),
        }
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                queue.dequeue().expect("Failed to dequeue");
            },
            Err(e) => println!("{}", e),
        }
    }

    queue.dispose(); // Disposing of the queue (not strictly necessary in Rust)
}
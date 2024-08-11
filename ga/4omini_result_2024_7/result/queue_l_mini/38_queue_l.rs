use std::io::{self, Write};
use std::ptr;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Use Vec to manage dynamic memory automatically
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, &'static str> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 1, // Start from 1 for easier mapping to C-style code
            rear: 0,
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            Err("Full queue")
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            Ok(self.array[self.front])
        }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(x)
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }
}

fn main() {
    let mut queue_length = String::new();
    print!("Enter the queue length: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut queue_length).expect("Failed to read line");
    let queue_length: usize = queue_length.trim().parse().expect("Invalid input");

    if queue_length < MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        std::process::exit(1);
    }

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue().expect("Failed to dequeue");
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32).expect("Failed to enqueue");
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let mut enqueue_value = String::new();
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut enqueue_value).expect("Failed to read line");
    
        let enqueue_value: i32 = enqueue_value.trim().parse().expect("Invalid input");
        
        queue.enqueue(enqueue_value).expect("Failed to enqueue");
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        let value = queue.front_and_dequeue().expect("Failed to front and dequeue");
        println!("FrontAndDequeue: {}", value);
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i as i32).expect("Failed to enqueue");
    }

    // Use front and dequeue
    while !queue.is_empty() {
        let front_value = queue.front().expect("Failed to get front");
        println!("Front: {}", front_value);
        queue.dequeue().expect("Failed to dequeue");
    }
}
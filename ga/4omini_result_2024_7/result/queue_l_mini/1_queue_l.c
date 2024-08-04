use std::io::{self, Write};
use std::ptr;
use std::mem;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Change to i32 for ElementType
}

impl Queue {
    pub fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let array = vec![0; max_elements]; // Initialize with zero values
        Ok(Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    pub fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }
        self.size += 1;
        self.rear = self.succ(self.rear);
        self.array[self.rear] = x;
        Ok(())
    }

    pub fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        Ok(self.array[self.front])
    }

    pub fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(())
    }

    pub fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        let x = self.array[self.front];
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(x)
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut queue_length = String::new();

    print!("Enter queue length: ");
    io::stdout().flush().unwrap(); // Flush stdout for prompt
    io::stdin().read_line(&mut queue_length).expect("Failed to read line");
    let queue_length: usize = match queue_length.trim().parse() {
        Ok(num) if num >= MIN_QUEUE_SIZE => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        },
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
        queue.dequeue().unwrap();
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i).unwrap();
        }
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        let mut enqueue_value = String::new();
        print!("Enter value to enqueue: ");
        io::stdout().flush().unwrap(); // Flush stdout for prompt
        io::stdin().read_line(&mut enqueue_value).expect("Failed to read line");
        let enqueue_value: i32 = match enqueue_value.trim().parse() {
            Ok(num) => num,
            _ => {
                eprintln!("Invalid input");
                return;
            },
        };
        queue.enqueue(enqueue_value).unwrap();
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
        queue.enqueue(i).unwrap();
    }

    // Use front and dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                queue.dequeue().unwrap();
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
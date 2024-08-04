use std::alloc::{GlobalAlloc, Layout};
use std::io::{self, Write};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Queue, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let array = vec![0; max_elements]; // Initialize with zeros
        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array,
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.array[self.rear] = x;
            self.rear = (self.rear + 1) % self.capacity;
            self.size += 1;
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, &'static str> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err("Empty queue")
        }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            let value = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Ok(value)
        }
    }
}

fn main() {
    let mut queue_length = String::new();
    print!("Enter the size of the queue: ");
    io::stdout().flush().unwrap(); // Ensure prompt is shown

    if io::stdin().read_line(&mut queue_length).is_err() {
        eprintln!("Failed to read line");
        return;
    }
    
    let queue_length: usize = match queue_length.trim().parse() {
        Ok(num) => num,
        Err(_) => {
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
        if let Err(e) = queue.dequeue() {
            eprintln!("{}", e);
        }
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
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
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap(); // Ensure prompt is shown

        if io::stdin().read_line(&mut enqueue_value).is_err() {
            eprintln!("Failed to read line");
            return;
        }

        let enqueue_value: i32 = match enqueue_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input");
                return;
            }
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
    for i in 0..(queue_length - 2) {
        queue.enqueue(i).unwrap();
    }

    // Use front and dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(e) => eprintln!("{}", e),
        }
        if let Err(e) = queue.dequeue() {
            eprintln!("{}", e);
        }
    }
}
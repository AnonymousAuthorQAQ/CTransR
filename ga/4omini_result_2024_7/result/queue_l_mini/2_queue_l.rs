use std::io::{self, Write};
use std::ptr;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn new(max_elements: usize) -> Result<Queue, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let array = vec![0; max_elements];

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
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

    pub fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }

        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
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
        self.front = (self.front + 1) % self.capacity;

        Ok(())
    }

    pub fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }

        let value = self.array[self.front];
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;

        Ok(value)
    }

    pub fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }
}

fn main() {
    let mut input = String::new();
    let mut queue: Queue;

    print!("Enter queue length: ");
    io::stdout().flush().unwrap(); // Ensure the prompt prints before reading input
    io::stdin().read_line(&mut input).unwrap();
    
    let queue_length: usize = match input.trim().parse() {
        Ok(num) if num >= Queue::MIN_QUEUE_SIZE => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    queue = Queue::new(queue_length).unwrap();

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue().unwrap();
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
    let mut enqueue_value = String::new();
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        print!("Enter value to enqueue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut enqueue_value).unwrap();
        
        let value: i32 = match enqueue_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input");
                return;
            }
        };

        queue.enqueue(value).unwrap();
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(err) => println!("{}", err),
        }
    }

    // Enqueue elements again
    for i in 0..(queue_length - 1) {
        queue.enqueue(i).unwrap();
    }

    // Use front and dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(err) => println!("{}", err),
        }
        queue.dequeue().unwrap();
    }
}
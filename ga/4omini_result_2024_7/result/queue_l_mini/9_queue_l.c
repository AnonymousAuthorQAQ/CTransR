use std::io::{self, Write};

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

    pub fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1, // Start at the end for wrapping
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

    pub fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
            Ok(())
        }
    }

    pub fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            Ok(self.array[self.front])
        }
    }

    pub fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            Ok(())
        }
    }

    pub fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        let front_value = self.front()?;
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        Ok(front_value)
    }

    pub fn dispose(self) {}
}

fn main() {
    let mut input = String::new();
    print!("Enter queue length (>= {}): ", Queue::MIN_QUEUE_SIZE);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().unwrap_or(0);

    if queue_length < Queue::MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        return;
    }

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue().expect("Failed to dequeue");
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
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut enqueue_value).unwrap();
        let value: i32 = enqueue_value.trim().parse().unwrap_or(0);
        queue.enqueue(value).expect("Failed to enqueue");
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
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Use front and dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(e) => eprintln!("{}", e),
        }
        queue.dequeue().expect("Failed to dequeue");
    }
}
use std::fmt;
use std::io::{self, Write};

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue<T> {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<T>,
}

impl<T> Queue<T> {
    // Creates a new queue with the specified maximum number of elements.
    fn create_queue(max_elements: usize) -> Result<Self, String> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }
        
        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array: Vec::with_capacity(max_elements),
        })
    }

    // Checks if the queue is empty.
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    // Checks if the queue is full.
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    // Adds an element to the rear of the queue.
    fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }

        if self.size < self.array.capacity() {
            self.array.push(item);
        } else {
            self.array[self.rear] = item;
        }
        
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;

        Ok(())
    }

    // Returns the front element.
    fn front(&self) -> Result<&T, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        Ok(&self.array[self.front])
    }

    // Removes an element from the front of the queue.
    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;

        Ok(())
    }

    // Gets the front element and removes it from the queue.
    fn front_and_dequeue(&mut self) -> Result<T, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        let item = std::mem::replace(&mut self.array[self.front], unsafe { std::mem::zeroed() });
        
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;

        Ok(item)
    }

    // Makes the queue empty.
    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }
}

// Main function to demonstrate queue functionalities.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    print!("Enter queue length: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let queue_length: usize = input.trim().parse()?;

    if queue_length < MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        std::process::exit(1);
    }

    let mut queue = Queue::create_queue(queue_length)?;

    // Attempt to dequeue from an empty queue.
    if !queue.is_empty() {
        queue.dequeue()?;
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full.
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i)?;
        }
    }

    // Test: Attempt to enqueue to a full queue.
    if !queue.is_full() {
        input.clear();
        print!("Enter a value to enqueue: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let enqueue_value: T = input.trim().parse()?;
        queue.enqueue(enqueue_value)?;
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements.
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => println!("{}", e),
        }
    }

    // Enqueue elements again.
    for i in 0..queue_length - 2 {
        queue.enqueue(i)?;
    }

    // Use front and dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(e) => println!("{}", e),
        }
        queue.dequeue()?;
    }

    Ok(())
}
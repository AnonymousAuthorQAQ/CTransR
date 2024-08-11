use std::io::{self, Write};
use std::ptr;

pub type ElementType = i32;

pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let array = vec![0; max_elements];

        Ok(Self {
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

    pub fn enqueue(&mut self, x: ElementType) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }

        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
        self.size += 1;
        Ok(())
    }

    pub fn front(&self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }

        Ok(self.array[self.front])
    }

    pub fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }

        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Ok(())
    }

    pub fn front_and_dequeue(&mut self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }

        let front_value = self.front();
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        front_value
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    print!("Enter queue length (>= 5): ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut input)?;
    let queue_length: usize = input.trim().parse()?;

    let mut queue = Queue::new(queue_length)?;

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue()?;
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i)?;
        }
    }

    // Test: Attempt to enqueue to a full queue
    input.clear();
    print!("Enter a value to enqueue: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    let enqueue_value: ElementType = input.trim().parse()?;
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        queue.enqueue(enqueue_value)?;
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue()?);
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        queue.enqueue(i)?;
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front()?);
        queue.dequeue()?;
    }

    Ok(())
}
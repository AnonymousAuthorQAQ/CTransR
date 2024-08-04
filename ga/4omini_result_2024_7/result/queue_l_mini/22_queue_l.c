use std::io::{self, Write};

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

    fn new(max_elements: usize) -> Result<Self, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }

        Ok(Self {
            capacity: max_elements,
            front: 0,
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

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }
        self.array[self.rear] = x;
        self.rear = (self.rear + 1) % self.capacity; // Circular increment
        self.size += 1;
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
        self.front = (self.front + 1) % self.capacity; // Circular increment
        self.size -= 1;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        let value = self.array[self.front];
        self.front = (self.front + 1) % self.capacity; // Circular increment
        self.size -= 1;
        Ok(value)
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter the size of the queue: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    
    let queue_length: usize = match input.trim().parse() {
        Ok(size) if size >= Queue::MIN_QUEUE_SIZE => size,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = Queue::new(queue_length).unwrap();

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        let _ = queue.dequeue(); // This won't be hit since the queue is empty.
    }

    // Enqueue elements
    for i in 0..queue_length - 1 {
        if let Err(err) = queue.enqueue(i as i32) {
            println!("Error: {}", err);
        }
    }

    // Test: Attempt to enqueue to a full queue
    input.clear();
    print!("Enter a value to enqueue: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else if let Ok(value) = input.trim().parse() {
        if let Err(err) = queue.enqueue(value) {
            println!("Error: {}", err);
        }
    } else {
        eprintln!("Invalid input");
        return;
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(err) => println!("Error: {}", err),
        }
    }

    // Example of enqueuing again
    for i in 0..queue_length - 2 {
        if let Err(err) = queue.enqueue(i as i32) {
            println!("Error: {}", err);
        }
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                let _ = queue.dequeue();
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
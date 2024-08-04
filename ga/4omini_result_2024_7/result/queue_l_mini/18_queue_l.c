use std::io::{self, Write};
use std::ptr;

type ElementType = i32;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        Ok(Self {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1, // Since we are using '0' indexing
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

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1; // Reset rear to the last element
    }

    fn enqueue(&mut self, x: ElementType) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.rear = self.successor(self.rear);
            self.array[self.rear] = x;
            self.size += 1;
            Ok(())
        }
    }

    fn front(&self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        Ok(self.array[self.front])
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            self.front = self.successor(self.front);
            self.size -= 1;
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        let x = self.array[self.front];
        self.front = self.successor(self.front);
        self.size -= 1;
        Ok(x)
    }

    fn successor(&self, value: usize) -> usize {
        (value + 1) % self.capacity
    }
}

const MIN_QUEUE_SIZE: usize = 5;

fn main() {
    let mut input = String::new();
    print!("Enter the queue size: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let queue_length: usize = match input.trim().parse() {
        Ok(num) if num >= MIN_QUEUE_SIZE => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = match Queue::new(queue_length) {
        Ok(q) => q,
        Err(err) => {
            eprintln!("{}", err);
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
        let mut enqueue_input = String::new();
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut enqueue_input).unwrap();

        if let Ok(value) = enqueue_input.trim().parse() {
            queue.enqueue(value).unwrap();
        } else {
            eprintln!("Invalid input");
            return;
        }
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(err) => eprintln!("{}", err),
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
            Err(err) => eprintln!("{}", err),
        }
        queue.dequeue().unwrap();
    }
}
use std::io::{self, Write};
use std::ptr;

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec to manage dynamic array
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, &'static str> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: vec![0; max_elements], // Initialize with zeros
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
        self.rear = self.capacity - 1;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            Err("Full queue")
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity; // Wrap around using modulo
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
            self.front = (self.front + 1) % self.capacity; // Wrap around using modulo
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity; // Wrap around using modulo
            Ok(x)
        }
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter queue length: ");
    io::stdout().flush().unwrap(); // Flush output to ensure prompt appears

    io::stdin().read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().unwrap_or(0);

    if queue_length < MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        std::process::exit(1);
    }

    let mut queue = Queue::new(queue_length).unwrap();

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue().unwrap(); // Safe since we checked
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
    if !queue.is_full() {
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap(); // Flush output to ensure prompt appears

        let mut enqueue_value = String::new();
        io::stdin().read_line(&mut enqueue_value).unwrap();
        let value: i32 = enqueue_value.trim().parse().unwrap_or(0);
        queue.enqueue(value).unwrap();
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
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

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                queue.dequeue().unwrap();
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
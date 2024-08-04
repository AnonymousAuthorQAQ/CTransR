use std::io::{self, Write};

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec for dynamic array
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }

        let array = vec![0; max_elements]; // Initialize the array with zeros

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
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

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        } else {
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
            self.size += 1;
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, String> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err("Empty queue".to_string())
        }
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        } else {
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        let x = self.front()?;
        self.dequeue()?;
        Ok(x)
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter queue length: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().unwrap_or(0);

    if queue_length < Queue::MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        return;
    }

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue().expect("Failed to dequeue");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i).expect("Failed to enqueue");
        }
    }

    // Test: Attempt to enqueue to a full queue
    print!("Enter a value to enqueue: ");
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let enqueue_value: i32 = input.trim().parse().unwrap_or(0);
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        queue.enqueue(enqueue_value).expect("Failed to enqueue");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(err) => eprintln!("{}", err),
        }
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(err) => eprintln!("{}", err),
        }
        queue.dequeue().expect("Failed to dequeue");
    }
}
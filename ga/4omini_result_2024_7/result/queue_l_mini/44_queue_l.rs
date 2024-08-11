use std::io::{self, Write};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec for dynamic array allocation
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Self, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
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
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
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
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        let value = self.array[self.front];
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Ok(value)
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the queue length: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    
    let queue_length: usize = match input.trim().parse() {
        Ok(num) if num >= Queue::MIN_QUEUE_SIZE => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = Queue::new(queue_length).unwrap();

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue().unwrap();
    } else {
        println!("Queue is empty, cannot dequeue.");
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
    if !queue.is_full() {
        input.clear();
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(value) = input.trim().parse() {
            queue.enqueue(value).unwrap();
        } else {
            eprintln!("Invalid input");
            return;
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => println!("{}", e),
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
            }
            Err(e) => println!("{}", e),
        }
    }
}
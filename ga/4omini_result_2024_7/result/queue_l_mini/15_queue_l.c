use std::io::{self, Write};
use std::ptr;
use std::mem;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Change to the desired type
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }
        
        let array = vec![0; max_elements]; // Initialize with zeros
        Ok(Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
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
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
            Ok(())
        }
    }

    pub fn front(&self) -> Result<i32, &'static str> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err("Empty queue")
        }
    }

    pub fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(())
        }
    }

    pub fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(x)
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    pub fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter queue length: ");
    io::stdout().flush().unwrap(); // Flush stdout to show the prompt
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
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        let _ = queue.dequeue();
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            let _ = queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    input.clear();
    print!("Enter value to enqueue: ");
    io::stdout().flush().unwrap(); // Flush stdout to show the prompt
    io::stdin().read_line(&mut input).unwrap();

    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else if let Ok(enqueue_value) = input.trim().parse::<i32>() {
        let _ = queue.enqueue(enqueue_value);
    } else {
        eprintln!("Invalid input");
        return;
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
        let _ = queue.enqueue(i as i32);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(err) => eprintln!("{}", err),
        }
        let _ = queue.dequeue();
    }
}
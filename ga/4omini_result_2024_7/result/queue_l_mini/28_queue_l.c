use std::io::{self, Write};
use std::ptr;

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

    fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 1,
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

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = self.successor(self.rear);
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
            self.front = self.successor(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.successor(self.front);
            Ok(x)
        }
    }

    fn successor(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter queue length: ");
    io::stdout().flush().unwrap(); // Flush the output buffer

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_length: usize = match input.trim().parse() {
        Ok(n) if n >= Queue::MIN_QUEUE_SIZE => n,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue().unwrap();
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
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        input.clear();
        print!("Enter value to enqueue: ");
        io::stdout().flush().unwrap(); // Flush the output buffer
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Ok(enqueue_value) = input.trim().parse::<i32>() {
            queue.enqueue(enqueue_value).expect("Failed to enqueue");
        } else {
            eprintln!("Invalid input");
            return;
        }
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => eprintln!("{}", e),
        }
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                queue.dequeue().expect("Failed to dequeue");
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
use std::io::{self, Write};

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
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
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

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: ElementType) -> Result<(), String> {
        if self.is_full() {
            Err("Full queue".to_string())
        } else {
            if self.size == 0 {
                self.array.push(x);
            } else {
                self.rear = (self.rear + 1) % self.capacity;
                if self.array.len() < self.rear + 1 {
                    self.array.push(0); // ensure capacity
                }
                self.array[self.rear] = x;
            }
            self.size += 1;
            Ok(())
        }
    }

    fn front(&self) -> Result<ElementType, String> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err("Empty queue".to_string())
        }
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            Err("Empty queue".to_string())
        } else {
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, String> {
        if self.is_empty() {
            Err("Empty queue".to_string())
        } else {
            let x = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Ok(x)
        }
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the queue length (minimum is 5): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().unwrap_or(0);

    if queue_length < Queue::MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        std::process::exit(1);
    }

    let mut queue = Queue::new(queue_length).unwrap();

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        if let Err(e) = queue.dequeue() {
            println!("{}", e);
        }
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            if let Err(e) = queue.enqueue(i) {
                println!("{}", e);
            }
        }
    }

    // Test: Attempt to enqueue to a full queue
    let mut input = String::new();
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(value) = input.trim().parse::<ElementType>() {
            if let Err(e) = queue.enqueue(value) {
                println!("{}", e);
            }
        } else {
            eprintln!("Invalid input");
            std::process::exit(1);
        }
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => println!("{}", e),
        }
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        if let Err(e) = queue.enqueue(i) {
            println!("{}", e);
        }
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                if let Err(e) = queue.dequeue() {
                    println!("{}", e);
                }
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
use std::io::{self, Write};

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using `i32` to represent `ElementType`
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
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

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }
        self.size += 1;
        self.rear = self.succ(self.rear);
        self.array[self.rear] = x;
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
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        let x = self.array[self.front];
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(x)
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter queue length (minimum {}): ", Queue::MIN_QUEUE_SIZE);
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input

    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let queue_length: usize = match input.trim().parse() {
        Ok(num) if num >= Queue::MIN_QUEUE_SIZE => num,
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
        if let Err(e) = queue.dequeue() {
            println!("{}", e);
        }
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            if let Err(e) = queue.enqueue(i as i32) {
                println!("{}", e);
            }
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let mut input = String::new();
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
        
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(enqueue_value) = input.trim().parse::<i32>() {
            if let Err(e) = queue.enqueue(enqueue_value) {
                println!("{}", e);
            }
        } else {
            eprintln!("Invalid input");
            return;
        }
    } else {
        println!("Queue is full, cannot enqueue.");
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
        if let Err(e) = queue.enqueue(i as i32) {
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
            Err(e) => println!("{}", e),
        }
    }
}
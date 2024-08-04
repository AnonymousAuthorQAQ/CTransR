use std::io::{self, Write};

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
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
        self.rear = self.capacity - 1;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
        self.size += 1;
        Ok(())
    }

    fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        Ok(self.array[self.front])
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        let value = self.array[self.front];
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Ok(value)
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the queue size: ");
    io::stdout().flush().unwrap(); // Flush to ensure prompt is printed

    io::stdin().read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().unwrap_or(0);
    
    if queue_length < MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        return;
    }

    let mut queue = match Queue::new(queue_length) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        if let Err(e) = queue.dequeue() {
            println!("Queue is empty, cannot dequeue: {}", e);
        }
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
            break;
        } else {
            if let Err(e) = queue.enqueue(i as i32) {
                println!("{}", e);
            }
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        input.clear();
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap(); // Flush to ensure prompt is printed

        io::stdin().read_line(&mut input).unwrap();
        let enqueue_value: i32 = input.trim().parse().unwrap_or(0);
        if let Err(e) = queue.enqueue(enqueue_value) {
            println!("{}", e);
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
        if let Err(e) = queue.enqueue(i as i32) {
            println!("{}", e);
        }
    }

    // Use front and dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(e) => println!("{}", e),
        }
        if let Err(e) = queue.dequeue() {
            println!("{}", e);
        }
    }
}
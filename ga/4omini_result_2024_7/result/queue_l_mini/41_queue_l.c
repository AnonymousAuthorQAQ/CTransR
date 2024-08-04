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
    fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < 5 {
            return Err("Queue size is too small");
        }
        Ok(Self {
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
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
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        let value = self.front()?;  // Attempt to get the front value
        self.dequeue()?;            // Then dequeue
        Ok(value)
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }
}

fn main() {
    print!("Enter the queue length: ");
    io::stdout().flush().unwrap(); // Ensure the prompt appears

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size too small");

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue().expect("This should not happen if the queue is empty");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32).expect("Failed to enqueue");
        }
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap(); // Ensure the prompt appears

        let mut input_value = String::new();
        io::stdin().read_line(&mut input_value).expect("Failed to read line");

        let enqueue_value: i32 = input_value.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value).expect("Failed to enqueue value");
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue().expect("Queue should not be empty"));
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        queue.enqueue(i as i32).expect("Failed to enqueue");
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front().expect("Queue should not be empty"));
        queue.dequeue().expect("Queue should not be empty");
    }
}
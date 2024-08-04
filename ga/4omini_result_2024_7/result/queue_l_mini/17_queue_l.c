use std::io;
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
    fn new(max_elements: usize) -> Result<Queue, &'static str> {
        if max_elements < 5 {
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
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
            self.size += 1;
            Ok(())
        }
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
        } else {
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Ok(())
        }
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
    
    println!("Enter the queue length (at least 5): ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_length: usize = match input.trim().parse() {
        Ok(num) if num >= 5 => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue().expect("Dequeue failed");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i).expect("Enqueue failed");
        }
    }

    // Attempt to enqueue to a full queue
    input.clear();
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        println!("Enter a value to enqueue: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value).expect("Enqueue failed");
    }

    // Front and dequeue
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(err) => eprintln!("{}", err),
        }
    }

    // Re-enqueue elements
    for i in 0..queue_length - 2 {
        queue.enqueue(i).expect("Enqueue failed");
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => println!("Front: {}", value),
            Err(err) => eprintln!("{}", err),
        }

        queue.dequeue().expect("Dequeue failed");
    }
}
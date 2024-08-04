use std::fmt;

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
    fn new(capacity: usize) -> Queue {
        if capacity < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; capacity],
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Queue is full");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = value;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let value = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(value)
        }
    }

    fn front(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.array[self.front])
        }
    }
}

fn main() {
    let mut queue_length = 0;
    println!("Enter the queue length (minimum 5):");
    loop {
        if let Ok(length) = std::io::stdin().read_line() {
            if let Ok(length) = length.trim().parse::<usize>() {
                if length >= MIN_QUEUE_SIZE {
                    queue_length = length;
                    break;
                } else {
                    println!("Queue size is too small, please enter a value of at least 5.");
                }
            } else {
                println!("Invalid input, please enter a number.");
            }
        }
    }

    let mut queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if let None = queue.dequeue() {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a value to enqueue:");
        if let Ok(input) = std::io::stdin().read_line() {
            if let Ok(value) = input.trim().parse::<i32>() {
                queue.enqueue(value);
            } else {
                println!("Invalid input");
            }
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front and dequeue
    while let Some(value) = queue.dequeue() {
        println!("Dequeued: {}", value);
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i as i32);
    }

    // Use front and dequeue
    while let Some(value) = queue.dequeue() {
        println!("Dequeued: {}", value);
    }
}
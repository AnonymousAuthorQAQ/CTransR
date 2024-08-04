use std::fmt;
use std::ops::{Index, IndexMut};
use std::ptr;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Box<[i32]>,
}

impl Queue {
    fn new(capacity: usize) -> Self {
        if capacity < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; capacity].into_boxed_slice(),
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
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
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

impl Index<usize> for Queue {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl IndexMut<usize> for Queue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.array)
    }
}

fn main() {
    let mut queue: Queue;
    let mut queue_length;

    loop {
        println!("Enter the queue length (>= {}):", MIN_QUEUE_SIZE);
        match std::io::stdin().read_line(&mut queue_length) {
            Ok(_) => {
                match queue_length.trim().parse::<usize>() {
                    Ok(length) => {
                        if length >= MIN_QUEUE_SIZE {
                            queue = Queue::new(length);
                            break;
                        } else {
                            println!("Queue size is too small, try again.");
                        }
                    }
                    Err(_) => println!("Invalid input, try again."),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("Queue: {}", queue);

    // Test: Attempt to dequeue from an empty queue
    if let Some(value) = queue.dequeue() {
        println!("Dequeued: {}", value);
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue.capacity - 1 {
        queue.enqueue(i);
        println!("Queue: {}", queue);
        if queue.is_full() {
            println!("Queue is full");
            break;
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a value to enqueue: ");
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            match input.trim().parse::<i32>() {
                Ok(value) => {
                    queue.enqueue(value);
                    println!("Enqueued: {}", value);
                }
                Err(_) => println!("Invalid input"),
            }
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front and dequeue
    while let Some(value) = queue.front() {
        println!("Front: {}", value);
        queue.dequeue();
        println!("Queue: {}", queue);
    }

    // Enqueue elements again
    for i in 0..queue.capacity - 2 {
        queue.enqueue(i);
    }

    // Use front and dequeue
    while let Some(value) = queue.front() {
        println!("Front: {}", value);
        queue.dequeue();
    }
}
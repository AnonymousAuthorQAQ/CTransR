use std::fmt;
use std::marker::PhantomData;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue<T> {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        if capacity < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity,
            front: 0,
            rear: 0,
            size: 0,
            elements: vec![T::default(); capacity],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, element: T) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.elements[self.rear] = element;
        self.rear = (self.rear + 1) % self.capacity;
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.elements[self.front])
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let value = self.elements[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(value)
        }
    }
}

impl<T: fmt::Display> fmt::Display for Queue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queue[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.elements[(self.front + i) % self.capacity])?;
        }
        write!(f, "]")
    }
}

fn main() {
    let mut queue_length: usize = MIN_QUEUE_SIZE;
    println!("Enter the queue size (at least {}): ", MIN_QUEUE_SIZE);
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");

    let mut queue: Queue<i32> = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue();
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let enqueue_value: i32 = 42; // Use a default value for demonstration
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front().unwrap());
        queue.dequeue();
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front().unwrap());
        queue.dequeue();
    }
}
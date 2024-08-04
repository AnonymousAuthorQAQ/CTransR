use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::fmt;

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
            elements: Vec::with_capacity(capacity),
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
            panic!("Queue is full");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.elements[self.rear] = element;
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
            let value = self.elements[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            Some(value)
        }
    }

    pub fn front_and_dequeue(&mut self) -> Option<T> {
        if let Some(value) = self.dequeue() {
            Some(value)
        } else {
            None
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Queue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Index<usize> for Queue<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for Queue<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

fn main() {
    let mut queue_length: usize = MIN_QUEUE_SIZE;
    println!("Enter the queue length (minimum 5):");
    std::io::stdin().read_line(&mut queue_length);

    let mut queue: Queue<i32> = Queue::new(queue_length);
    println!("Initial empty queue: {:?}", queue);

    // Test: Dequeue from empty queue
    if let None = queue.dequeue() {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements
    for i in 0..queue.capacity - 1 {
        queue.enqueue(i);
    }

    // Test: Enqueue to full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        let enqueue_value: i32;
        println!("Enter an integer to enqueue:");
        std::io::stdin().read_line(&mut enqueue_value);
        queue.enqueue(enqueue_value);
    }

    // FrontAndDequeue elements
    println!("FrontAndDequeue:");
    while let Some(value) = queue.front_and_dequeue() {
        println!("{}", value);
    }

    // Enqueue elements again
    for i in 0..queue.capacity - 2 {
        queue.enqueue(i);
    }

    // Front and Dequeue
    println!("Front and Dequeue:");
    while let Some(value) = queue.dequeue() {
        println!("{}", value);
    }

    println!("Disposed queue");
}

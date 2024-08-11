use std::fmt;
use std::ops::{Index, IndexMut};

const MIN_QUEUE_SIZE: usize = 5;

struct Queue<T> {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<T>,
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
            array: Vec::with_capacity(capacity),
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
        self.array.push(element);
        self.rear = (self.rear + 1) % self.capacity;
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.array[self.front])
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let value = self.array.remove(self.front);
            self.front = (self.front + 1) % self.capacity;
            Some(value)
        }
    }
}

impl<T> fmt::Display for Queue<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = self.clone();
        while let Some(value) = queue.dequeue() {
            write!(f, "{}\n", value)?;
        }
        Ok(())
    }
}

fn main() {
    let mut queue: Queue<i32> = Queue::new(12);
    for i in 0..10 {
        queue.enqueue(i);
    }
    println!("{}", queue);

    while !queue.is_empty() {
        println!("{}", queue.dequeue().unwrap());
    }

    for i in 0..10 {
        queue.enqueue(i);
    }
    println!("{}", queue);

    while !queue.is_empty() {
        println!("{}", queue.dequeue().unwrap());
    }
}
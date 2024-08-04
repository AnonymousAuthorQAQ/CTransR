use std::fmt;
use std::mem;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
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
            array: vec![T::default(); capacity],
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
        self.array[self.rear] = element;
        self.rear = (self.rear + 1) % self.capacity;
    }

    pub fn front(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.array[self.front])
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let front = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(front)
        }
    }
}

impl<T: fmt::Display> fmt::Display for Queue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queue {{ capacity: {}, front: {}, rear: {}, size: {}, array: {:?} }}", self.capacity, self.front, self.rear, self.size, self.array)
    }
}

fn main() {
    let mut q: Queue<i32> = Queue::new(12);

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q.dequeue().unwrap());
    }

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q.dequeue().unwrap());
    }
}
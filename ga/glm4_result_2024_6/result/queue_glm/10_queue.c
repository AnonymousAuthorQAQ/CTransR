use std::ptr;
use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Result};

const MIN_QUEUE_SIZE: usize = 5;

struct Queue<T> {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Box<[T]>,
}

impl<T> Queue<T> {
    fn new(max_elements: usize) -> Self {
        assert!(max_elements >= MIN_QUEUE_SIZE, "Queue size is too small");
        Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![T::default(); max_elements].into_boxed_slice(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, elem: T) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.array[self.rear] = elem;
        self.rear = (self.rear + 1) % self.capacity;
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let value = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            Some(value)
        }
    }

    fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.array[self.front])
        }
    }
}

impl<T: Display> Display for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<()> {
        for i in 0..self.size {
            write!(f, "{} ", self.array[(self.front + i) % self.capacity])?;
        }
        Ok(())
    }
}

fn main() {
    let mut q = Queue::new(12);

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q);
        q.dequeue();
    }

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q);
        q.dequeue();
    }
}
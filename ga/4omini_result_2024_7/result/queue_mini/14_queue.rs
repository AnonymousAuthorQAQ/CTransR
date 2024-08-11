use std::mem;
use std::ptr;
use std::fmt;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: Vec<i32>, // Assuming ElementType is i32
}

#[derive(Debug)]
pub enum QueueError {
    EmptyQueue,
    FullQueue,
    OutOfMemory,
}

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, QueueError> {
        if max_elements < 5 {
            return Err(QueueError::OutOfMemory);
        }

        Ok(Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements], // Pre-allocate array
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
        self.rear = -1;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), QueueError> {
        if self.is_full() {
            return Err(QueueError::FullQueue);
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear as usize] = x;
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, QueueError> {
        if self.is_empty() {
            return Err(QueueError::EmptyQueue);
        }
        Ok(self.array[self.front])
    }

    fn dequeue(&mut self) -> Result<(), QueueError> {
        if self.is_empty() {
            return Err(QueueError::EmptyQueue);
        } else {
            self.size -= 1;
            self.front = self.succ(self.front as isize) as usize; // Cast to usize
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, QueueError> {
        if self.is_empty() {
            return Err(QueueError::EmptyQueue);
        }
        let x = self.array[self.front];
        self.size -= 1;
        self.front = self.succ(self.front as isize) as usize; // Cast to usize
        Ok(x)
    }

    fn succ(&self, value: isize) -> isize {
        let new_value = value + 1;
        if new_value as usize == self.capacity {
            0
        } else {
            new_value
        }
    }
}

fn main() {
    let mut queue = Queue::new(12).expect("Failed to create a queue");

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        println!("{}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        println!("{}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }
}
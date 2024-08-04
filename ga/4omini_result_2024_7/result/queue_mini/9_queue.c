use std::error::Error;
use std::fmt;
use std::mem;

#[derive(Debug)]
struct QueueError;

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue error")
    }
}

impl Error for QueueError {}

type ElementType = i32;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Self, Box<dyn Error>> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err(Box::new(QueueError));
        }

        Ok(Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements], // Initialize with zeros
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: ElementType) -> Result<(), Box<dyn Error>> {
        if self.is_full() {
            return Err(Box::new(QueueError));
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
            Ok(())
        }
    }

    fn front(&self) -> Result<ElementType, Box<dyn Error>> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err(Box::new(QueueError))
        }
    }

    fn dequeue(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_empty() {
            Err(Box::new(QueueError))
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(QueueError));
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(x)
        }
    }

    fn succ(&self, value: usize) -> usize {
        (value + 1) % self.capacity
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut queue = Queue::new(12)?;

    for i in 0..10 {
        queue.enqueue(i)?;
    }

    while !queue.is_empty() {
        println!("{}", queue.front()?);
        queue.dequeue()?;
    }

    for i in 0..10 {
        queue.enqueue(i)?;
    }

    while !queue.is_empty() {
        println!("{}", queue.front()?);
        queue.dequeue()?;
    }

    Ok(())
}
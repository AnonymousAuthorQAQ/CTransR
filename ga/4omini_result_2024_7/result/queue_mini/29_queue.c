use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct QueueError {
    details: String,
}

impl QueueError {
    fn new(msg: &str) -> QueueError {
        QueueError {
            details: String::from(msg),
        }
    }
}

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QueueError: {}", self.details)
    }
}

impl Error for QueueError {
    fn description(&self) -> &str {
        &self.details
    }
}

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

    fn create_queue(max_elements: usize) -> Result<Self, QueueError> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err(QueueError::new("Queue size is too small"));
        }

        let array = Vec::with_capacity(max_elements);
        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array,
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
        self.front = 0;
        self.rear = self.capacity - 1;
    }

    fn enqueue(&mut self, x: ElementType) -> Result<(), QueueError> {
        if self.is_full() {
            Err(QueueError::new("Full queue"))
        } else {
            self.size += 1;
            self.rear = self.successor(self.rear);
            self.array.push(x); // Push to the end of the array
            Ok(())
        }
    }

    fn front(&self) -> Result<&ElementType, QueueError> {
        if self.is_empty() {
            Err(QueueError::new("Empty queue"))
        } else {
            Ok(&self.array[self.front])
        }
    }

    fn dequeue(&mut self) -> Result<(), QueueError> {
        if self.is_empty() {
            Err(QueueError::new("Empty queue"))
        } else {
            self.size -= 1;
            self.front = self.successor(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, QueueError> {
        if self.is_empty() {
            Err(QueueError::new("Empty queue"))
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.successor(self.front);
            Ok(x)
        }
    }

    fn successor(&self, value: usize) -> usize {
        (value + 1) % self.capacity
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut q = Queue::create_queue(12)?;

    for i in 0..10 {
        q.enqueue(i)?;
    }

    while !q.is_empty() {
        println!("{}", q.front()?);
        q.dequeue()?;
    }

    for i in 0..10 {
        q.enqueue(i)?;
    }

    while !q.is_empty() {
        println!("{}", q.front()?);
        q.dequeue()?;
    }

    Ok(())
}
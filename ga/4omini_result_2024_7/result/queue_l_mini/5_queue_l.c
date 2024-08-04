use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct QueueError;

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation on an empty or full queue.")
    }
}

impl Error for QueueError {}

type ElementType = i32;
const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, Box<dyn Error>> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err(Box::new(QueueError));
        }

        Ok(Self {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements],
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
        self.rear = 0;
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
            Err(Box::new(QueueError))
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(x)
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut queue_length = String::new();
    std::io::stdin().read_line(&mut queue_length)?;
    let queue_length: usize = queue_length.trim().parse()?;
    
    if queue_length < MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        return Err(Box::new(QueueError));
    }

    let mut queue = Queue::new(queue_length)?;

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i)?;
        }
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        let mut enqueue_value = String::new();
        std::io::stdin().read_line(&mut enqueue_value)?;
        let enqueue_value: ElementType = enqueue_value.trim().parse()?;
        queue.enqueue(enqueue_value)?;
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue()?);
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i)?;
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front()?);
        queue.dequeue()?;
    }

    Ok(())
}
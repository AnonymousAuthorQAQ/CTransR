use std::error::Error;
use std::fmt;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Assuming the ElementType is Integer
}

#[derive(Debug)]
struct QueueError(String);

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QueueError: {}", self.0)
    }
}

impl Error for QueueError {}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, Box<dyn Error>> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err(Box::new(QueueError("Queue size is too small".to_string())));
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
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

    fn enqueue(&mut self, x: i32) -> Result<(), Box<dyn Error>> {
        if self.is_full() {
            return Err(Box::new(QueueError("Full queue".to_string())));
        }

        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
        Ok(())
    }

    fn front(&self) -> Result<i32, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(QueueError("Empty queue".to_string())));
        }
        Ok(self.array[self.front])
    }

    fn dequeue(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(QueueError("Empty queue".to_string())));
        }

        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(QueueError("Empty queue".to_string())));
        }

        let value = self.array[self.front];
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        Ok(value)
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    use std::io;
    use std::num::ParseIntError;

    let mut input = String::new();

    print!("Enter queue length: ");
    io::stdin().read_line(&mut input)?;
    let queue_length: usize = input.trim().parse().map_err(|e: ParseIntError| Box::new(e))?;

    let mut queue = Queue::new(queue_length)?;

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue()?;
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i)?;
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        input.clear();
        print!("Enter an enqueue value: ");
        io::stdin().read_line(&mut input)?;
        let enqueue_value: i32 = input.trim().parse().map_err(|e: ParseIntError| Box::new(e))?;
        queue.enqueue(enqueue_value)?;
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue()?);
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        queue.enqueue(i)?;
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front()?);
        queue.dequeue()?;
    }

    Ok(())
}
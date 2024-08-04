use std::io::{self, Write};

type ElementType = i32;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, &'static str> {
        const MIN_QUEUE_SIZE: usize = 5;
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }
        
        let array = vec![0; max_elements];
        Ok(Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
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
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: ElementType) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = self.successor(self.rear);
            self.array[self.rear] = x;
        }
        Ok(())
    }

    fn front(&self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        Ok(self.array[self.front])
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.successor(self.front);
        }
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        let x = self.array[self.front];
        self.size -= 1;
        self.front = self.successor(self.front);
        Ok(x)
    }

    fn successor(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() -> io::Result<()> {
    let mut queue = Queue::new(12).expect("Failed to create queue");
    
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

    Ok(())
}
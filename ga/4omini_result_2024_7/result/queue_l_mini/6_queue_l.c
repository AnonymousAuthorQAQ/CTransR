use std::io;
use std::alloc::{alloc, dealloc, Layout};

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }

        let layout = Layout::from_size_align(max_elements * std::mem::size_of::<i32>(), std::mem::align_of::<i32>())
            .map_err(|_| "Failed to create layout".to_string())?;

        let array = unsafe { alloc(layout) };
        if array.is_null() {
            return Err("Out of space!!!".to_string());
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array,
        })
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        unsafe {
            *(self.array.add(self.rear)) = x;
        }
        Ok(())
    }

    fn front(&self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        unsafe {
            Ok(*(self.array.add(self.front)))
        }
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        let value = self.front()?;
        self.dequeue()?;
        Ok(value)
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.capacity * std::mem::size_of::<i32>(), std::mem::align_of::<i32>())
                .unwrap();
            dealloc(self.array as *mut u8, layout);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let queue_length: usize = match input.trim().parse() {
        Ok(n) if n >= MIN_QUEUE_SIZE => n,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");
    queue.make_empty();

    // Attempt to dequeue from an empty queue
    match queue.dequeue() {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i).expect("Failed to enqueue");
        }
    }

    // Attempt to enqueue to a full queue
    match queue.enqueue(queue_length as i32) {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue().expect("Failed to front and dequeue"));
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }
}
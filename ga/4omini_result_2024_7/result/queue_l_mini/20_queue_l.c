use std::io;
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32, // Using raw pointer for dynamic array allocation
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let layout = Layout::from_size_align(max_elements * mem::size_of::<i32>(), mem::align_of::<i32>())
            .expect("Failed to create layout");
        
        unsafe {
            let array = alloc(layout);
            if array.is_null() {
                return Err("Out of space!!!");
            }

            Ok(Queue {
                capacity: max_elements,
                front: 0,
                rear: (max_elements - 1),
                size: 0,
                array,
            })
        }
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            unsafe {
                *(self.array.offset(self.rear as isize)) = x;
            }
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        unsafe {
            Ok(*(self.array.offset(self.front as isize)))
        }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        let value = self.front()?;
        self.dequeue()?;
        Ok(value)
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = Layout::from_size_align(self.capacity * mem::size_of::<i32>(), mem::align_of::<i32>())
                    .expect("Failed to create layout");
                dealloc(self.array, layout);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter queue length:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size is too small");

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue().expect("Failed to dequeue");
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i).expect("Failed to enqueue");
        }
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        println!("Enter a value to enqueue:");
        let mut value_input = String::new();
        io::stdin().read_line(&mut value_input).expect("Failed to read line");
        let enqueue_value: i32 = value_input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value).expect("Failed to enqueue");
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => println!("{}", e),
        }
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Use front and dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                queue.dequeue().expect("Failed to dequeue");
            }
            Err(e) => println!("{}", e),
        }
    }
}
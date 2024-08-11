use std::alloc::{self, Layout};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: *mut i32, // Using raw pointer for dynamic array
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let layout = Layout::array::<i32>(max_elements).map_err(|_| "Out of space!!!")?;
        let array = unsafe { alloc::alloc(layout) };

        if array.is_null() {
            return Err("Out of space!!!");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: -1,
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity as isize;
            unsafe {
                ptr::write(self.array.add(self.rear as usize), x);
            }
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        unsafe {
            Ok(ptr::read(self.array.add(self.front)))
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
        let x = self.front()?;
        self.dequeue()?;
        Ok(x)
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = -1;
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = Layout::array::<i32>(self.capacity).unwrap();
                alloc::dealloc(self.array, layout);
            }
        }
    }
}

fn main() {
    let mut queue_length = String::new();
    std::io::stdin().read_line(&mut queue_length).expect("Failed to read line");
    let queue_length: usize = queue_length.trim().parse().expect("Invalid input");

    if queue_length < Queue::MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        std::process::exit(1);
    }

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue().expect("Failed to dequeue");
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i).expect("Failed to enqueue");
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let mut enqueue_value = String::new();
        std::io::stdin().read_line(&mut enqueue_value).expect("Failed to read line");
        let enqueue_value: i32 = enqueue_value.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value).expect("Failed to enqueue");
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue().expect("Failed to front and dequeue"));
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }
}
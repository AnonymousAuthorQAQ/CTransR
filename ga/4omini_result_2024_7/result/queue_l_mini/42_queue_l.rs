use std::alloc::{self, Layout};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: *mut i32, // used i32 for `ElementType`
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, &'static str> {
        const MIN_QUEUE_SIZE: usize = 5;

        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        // Allocate memory for the Queue
        let layout = Layout::from_size_align(std::mem::size_of::<i32>() * max_elements, std::mem::align_of::<i32>()).unwrap();
        let array = unsafe { alloc::alloc(layout) as *mut i32 };

        if array.is_null() {
            return Err("Out of space!!!");
        }

        let queue = Queue {
            capacity: max_elements,
            front: 1,
            rear: -1,
            size: 0,
            array,
        };

        Ok(queue)
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = self.successor(self.rear);
            unsafe {
                *self.array.add(self.rear as usize) = x;
            }
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            unsafe { Ok(*self.array.add(self.front as usize)) }
        }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            self.size -= 1;
            self.front = self.successor(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            let value = unsafe { *self.array.add(self.front as usize) };
            self.size -= 1;
            self.front = self.successor(self.front);
            Ok(value)
        }
    }

    fn successor(&self, value: isize) -> usize {
        let mut new_value = value + 1;
        if new_value as usize == self.capacity {
            new_value = 0;
        }
        new_value as usize
    }

    fn dispose(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = Layout::from_size_align(std::mem::size_of::<i32>() * self.capacity, std::mem::align_of::<i32>()).unwrap();
                alloc::dealloc(self.array as *mut u8, layout);
            }
        }
    }
}

fn main() {
    let mut queue_length = String::new();
    std::io::stdin().read_line(&mut queue_length).unwrap();
    let queue_length: usize = queue_length.trim().parse().unwrap_or(0);

    if queue_length < 5 {
        eprintln!("Invalid input or queue size is too small");
        std::process::exit(1);
    }

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue().expect("Unexpected error");
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32).expect("Unexpected error");
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let mut enqueue_value = String::new();
        std::io::stdin().read_line(&mut enqueue_value).unwrap();
        let enqueue_value: i32 = enqueue_value.trim().parse().unwrap_or(0);
        queue.enqueue(enqueue_value).expect("Unexpected error");
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(err) => eprintln!("{}", err),
        }
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        queue.enqueue(i as i32).expect("Unexpected error");
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                queue.dequeue().expect("Unexpected error");
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    queue.dispose();
}
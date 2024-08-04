use std::alloc::{GlobalAlloc, System, Layout};
use std::ptr;
use std::io;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: *mut i32, // Change ElementType/Type as per need
}

impl Queue {
    const MIN_QUEUE_SIZE:usize = 5;

    fn new(max_elements: usize) -> Option<Queue> {
        if max_elements < Queue::MIN_QUEUE_SIZE {
            eprintln!("Queue size is too small");
            return None;
        }

        unsafe {
            let layout = Layout::array::<i32>(max_elements).unwrap();
            let array = System.alloc(layout);

            if array.is_null() {
                eprintln!("Out of space!!!");
                return None;
            }

            Some(Queue {
                capacity: max_elements,
                front: 0, 
                rear: 0, // Start with rear not pointing to any element
                size: 0,
                array
            })
        }
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = -1; // Specifically meaning it no longer points to a member
    }

    fn dispose(&self) {
        if !self.array.is_null() {
            unsafe {
                let layout = Layout::array::<i32>(self.capacity).unwrap();
                System.dealloc(self.array, layout);
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear as usize) as isize;
            unsafe {
                *self.array.add(self.rear as usize) = x;
            }
        }
    }

    fn front(&self) -> i32 {
        if !self.is_empty() {
            unsafe {
                *self.array.add(self.front) 
            }
        } else {
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = unsafe { *self.array.add(self.front) };
            self.size -= 1;
            self.front = self.succ(self.front);
            x
        }
    }
}

fn main() {
    let mut queue_length = String::new();
    io::stdin()
        .read_line(&mut queue_length)
        .expect("Failed to read input");
    
    let queue_length: usize = queue_length.trim().parse().expect("Invalid length");

    if queue_length < Queue::MIN_QUEUE_SIZE {
        eprintln!("Invalid input or queue size is too small");
        return;
    }

    let mut queue = Queue::new(queue_length).expect("Failed to create the queue");

    // Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue();
    }

    // Fill the queue
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32);
        }
    }

    // Attempt to enqueue to a full queue
    println!("Please enter a value to enqueue:");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");
    let enqueue_value: i32 = value.trim().parse().expect("Invalid input");

    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        queue.enqueue(enqueue_value);
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Re-enqueue elements
    for i in 0..queue_length - 2 {
        queue.enqueue(i as i32);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }

    queue.dispose();
}
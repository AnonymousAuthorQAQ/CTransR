use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::io::{self, Write};

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32, // Using raw pointer for the array to manage memory manually
}

impl Queue {
    fn create(max_elements: usize) -> Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        let layout = Layout::array::<i32>(max_elements).unwrap();
        let array_ptr = unsafe { alloc(layout) as *mut i32 }; // Allocate memory
        if array_ptr.is_null() {
            panic!("Out of space!!!");
        }

        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1, // Start rear at the last index
            size: 0,
            array: array_ptr,
        }
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1; // rear at the last index
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity; // Circular increment
        unsafe {
            *self.array.add(self.rear) = x; // Add the element
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        unsafe {
            *self.array.add(self.front) // Access the front element
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity; // Circular increment
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front(); // Get front element
        self.dequeue(); // Then dequeue it
        x
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<i32>(self.capacity).unwrap();
            dealloc(self.array as *mut u8, layout); // Deallocate the memory
        }
    }
}

fn main() {
    let mut q = Queue::create(12);

    // Enqueue elements 0 to 9
    for i in 0..10 {
        q.enqueue(i);
    }

    // Dequeue and print until empty
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }

    // Enqueue elements 0 to 9 again
    for i in 0..10 {
        q.enqueue(i);
    }

    // Dequeue and print until empty again
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
}
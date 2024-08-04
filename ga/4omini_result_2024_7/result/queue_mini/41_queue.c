use std::alloc::{self, Layout};
use std::ptr;
use std::panic;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    // CreateQueue: Initialize a new queue with a specific capacity
    pub fn create_queue(max_elements: usize) -> *mut Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        // Allocate memory for the queue structure
        let layout = Layout::new::<Queue>();
        let queue_ptr = unsafe { alloc::alloc(layout) as *mut Queue };
        if queue_ptr.is_null() {
            panic!("Out of memory!");
        }

        let layout_array = Layout::array::<i32>(max_elements).unwrap();
        let array_ptr = unsafe { alloc::alloc(layout_array) };
        if array_ptr.is_null() {
            unsafe { alloc::dealloc(queue_ptr as *mut u8, layout) };
            panic!("Out of memory!");
        }

        // Initialize the queue structure
        unsafe {
            (*queue_ptr).capacity = max_elements;
            (*queue_ptr).size = 0;
            (*queue_ptr).front = 0;
            (*queue_ptr).rear = max_elements - 1;
            (*queue_ptr).array = array_ptr;
        }

        queue_ptr
    }

    // MakeEmpty: Reset the queue to an empty state
    pub fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1; // Points to the last position
    }

    // IsEmpty: Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // IsFull: Check if the queue is full
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    // Enqueue: Insert an element into the queue
    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }

        self.size += 1;
        self.rear = self.succ(self.rear);
        unsafe {
            *self.array.offset(self.rear as isize) = x;
        }
    }

    // Dequeue: Remove an element from the front of the queue
    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }

        self.size -= 1;
        self.front = self.succ(self.front);
    }

    // Front: Retrieve the front element of the queue
    pub fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        unsafe { *self.array.offset(self.front as isize) }
    }

    // FrontAndDequeue: Retrieve and remove the front element of the queue
    pub fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front();
        self.dequeue();
        x
    }

    // Deallocation function to dispose of the queue
    pub unsafe fn dispose_queue(&mut self) {
        if !self.array.is_null() {
            alloc::dealloc(self.array as *mut u8, Layout::array::<i32>(self.capacity).unwrap());
        }
        let layout = Layout::new::<Queue>();
        alloc::dealloc(self as *mut _ as *mut u8, layout);
    }

    // Returns the next index in a circular manner
    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut queue_ptr = Queue::create_queue(12);
    let queue: &mut Queue = unsafe { &mut *queue_ptr };

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    unsafe {
        queue.dispose_queue();
    }
}
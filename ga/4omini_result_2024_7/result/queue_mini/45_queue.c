use std::alloc::{self, Layout};
use std::ptr;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    pub fn new(max_elements: usize) -> Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        let layout = Layout::array::<i32>(max_elements).unwrap();
        let array = unsafe { alloc::alloc(layout) };

        if array.is_null() {
            panic!("Out of space!!!");
        }

        Queue {
            capacity: max_elements,
            front: 0,
            rear: -1,
            size: 0,
            array,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity as isize;
            unsafe {
                ptr::write(self.array.add(self.rear as usize), x);
            }
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        unsafe { *self.array.add(self.front) }
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = self.front();
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            x
        }
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
    let mut q = Queue::new(12);

    // Enqueue integers 0 to 9
    for i in 0..10 {
        q.enqueue(i);
    }

    // Dequeue and print
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }

    // Enqueue integers 0 to 9 again
    for i in 0..10 {
        q.enqueue(i);
    }

    // Dequeue and print again
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
}
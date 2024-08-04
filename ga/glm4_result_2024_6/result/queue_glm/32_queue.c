use std::ptr::null_mut;
use std::fmt::Error;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    fn new(max_elements: usize) -> Self {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        let mut array = Vec::with_capacity(max_elements);
        array.resize(max_elements, 0);
        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: array.as_mut_ptr(),
        }
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
        self.rear = (self.rear + 1) % self.capacity;
        unsafe { *self.array.offset(self.rear as isize) = x };
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        unsafe { *self.array.offset(self.front as isize) }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                Vec::from_raw_parts(self.array, self.capacity, self.capacity);
            }
        }
    }
}

fn main() {
    let mut q = Queue::new(12);
    for i in 0..10 {
        q.enqueue(i);
    }
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
    for i in 0..10 {
        q.enqueue(i);
    }
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
}
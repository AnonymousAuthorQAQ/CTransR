use std::alloc::{self, Layout};
use std::ptr;
use std::mem;
use std::io::{self, Write};

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut ElementType,
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        let layout = Layout::array::<ElementType>(max_elements).unwrap();
        let array = unsafe { alloc::alloc(layout) };
        if array.is_null() {
            panic!("Out of space!!!");
        }

        Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.successor(self.rear);
            unsafe {
                ptr::write(self.array.add(self.rear), x);
            }
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            unsafe { *self.array.add(self.front) }
        } else {
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.successor(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = unsafe { *self.array.add(self.front) };
            self.size -= 1;
            self.front = self.successor(self.front);
            x
        }
    }

    fn successor(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                let layout = Layout::array::<ElementType>(self.capacity).unwrap();
                alloc::dealloc(self.array, layout);
            }
        }
    }
}

fn main() {
    let mut queue = Queue::new(12);

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
}
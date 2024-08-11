use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn new(max_elements: usize) -> Queue {
        if max_elements < Self::MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        let layout = Layout::array::<i32>(max_elements).expect("Failed to create layout");
        let array = unsafe { alloc(layout) as *mut i32 };

        if array.is_null() {
            panic!("Out of space!!!");
        }

        Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
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
            self.rear = self.succ(self.rear);
            unsafe {
                ptr::write(self.array.add(self.rear), x);
            }
        }
    }

    pub fn front(&self) -> i32 {
        if !self.is_empty() {
            unsafe { *self.array.add(self.front) }
        } else {
            panic!("Empty queue");
        }
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = unsafe { *self.array.add(self.front) };
            self.size -= 1;
            self.front = self.succ(self.front);
            x
        }
    }

    fn succ(&self, value: usize) -> usize {
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
                let layout = Layout::array::<i32>(self.capacity).expect("Failed to create layout for deallocation");
                dealloc(self.array as *mut u8, layout);
            }
        }
    }
}

fn main() {
    let mut queue = Queue::new(12);

    for i in 0..10 {
        queue.enqueue(i as i32);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    for i in 0..10 {
        queue.enqueue(i as i32);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }
}
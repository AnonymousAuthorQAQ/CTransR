use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: isize, // Using isize to manage wrapping of Rear
    size: usize,
    array: *mut i32,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn create_queue(max_elements: usize) -> Queue {
        if max_elements < Self::MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        let layout = Layout::array::<i32>(max_elements).expect("Failed to create layout");
        let array = unsafe { alloc(layout) };

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

    pub fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = -1;
    }

    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % (self.capacity as isize);
            unsafe {
                ptr::write(self.array.add(self.rear as usize), x);
            }
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        unsafe {
            *self.array.add(self.front as usize)
        }
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
        let x = self.front();
        self.dequeue();
        x
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<i32>(self.capacity).expect("Failed to create layout");
            dealloc(self.array, layout);
        }
    }
}

fn main() {
    let mut queue = Queue::create_queue(12);

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
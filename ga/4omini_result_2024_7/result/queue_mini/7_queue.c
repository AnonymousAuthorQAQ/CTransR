use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: *mut i32, // Using *mut for raw pointer to mimic C's dynamic array behavior
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }

        let layout = Layout::from_size_align(max_elements * std::mem::size_of::<i32>(), std::mem::align_of::<i32>()).unwrap();
        let array = unsafe { alloc(layout) as *mut i32 };

        if array.is_null() {
            panic!("Out of space!!!");
        }

        Queue {
            capacity: max_elements,
            front: 1,
            rear: -1,
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

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = -1;
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity as isize;
            unsafe {
                *self.array.add(self.rear as usize) = x;
            }
        }
    }

    fn front(&self) -> i32 {
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
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
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
            let layout = Layout::from_size_align(self.capacity * std::mem::size_of::<i32>(), std::mem::align_of::<i32>()).unwrap();
            unsafe { dealloc(self.array as *mut u8, layout) }
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
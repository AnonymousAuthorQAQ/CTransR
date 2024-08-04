use std::ptr::null_mut;
use std::fmt;
use std::mem;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    fn new(capacity: usize) -> Queue {
        if capacity < 5 {
            panic!("Queue size is too small");
        }
        let array = unsafe { Box::into_raw(Box::new([0; capacity])) };
        Queue {
            capacity,
            front: 1,
            rear: 0,
            size: 0,
            array,
        }
    }

    fn dispose(&mut self) {
        if self.array != null_mut() {
            unsafe {
                Box::from_raw(self.array);
            }
            self.array = null_mut();
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
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            unsafe {
                *self.array.offset(self.rear as isize) = x;
            }
        }
    }

    fn front(&self) -> i32 {
        if !self.is_empty() {
            unsafe { *self.array.offset(self.front as isize) }
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
            self.size -= 1;
            let front = self.front;
            self.front = (self.front + 1) % self.capacity;
            unsafe { *self.array.offset(front as isize) }
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        self.dispose();
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
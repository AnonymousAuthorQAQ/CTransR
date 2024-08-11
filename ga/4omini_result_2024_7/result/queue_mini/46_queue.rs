use std::alloc::{self, handle_alloc_error};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    fn new(max_elements: usize) -> Box<Self> {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }

        let array = unsafe {
            let layout = std::alloc::Layout::array::<i32>(max_elements).unwrap();
            let ptr = alloc::alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            ptr
        };

        Box::new(Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array,
        })
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
        self.rear = self.succ(self.rear);
        unsafe {
            *(self.array.add(self.rear)) = x;
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        unsafe { *(self.array.add(self.front)) }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = self.succ(self.front);
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let x;
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            x = self.front();
            self.front = self.succ(self.front);
        }
        x
    }

    fn succ(&self, value: usize) -> usize {
        let mut value = value + 1;
        if value == self.capacity {
            value = 0;
        }
        value
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            let layout = std::alloc::Layout::array::<i32>(self.capacity).unwrap();
            alloc::dealloc(self.array, layout);
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
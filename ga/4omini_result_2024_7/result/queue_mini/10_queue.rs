use std::alloc::{dealloc, Layout};
use std::ptr::null_mut;
use std::process;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32, // Using raw pointer for dynamic array
}

impl Queue {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        let layout = Layout::array::<i32>(max_elements).unwrap();
        let array = unsafe { std::alloc::alloc(layout) };

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

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn dispose_queue(&mut self) {
        if !self.array.is_null() {
            let layout = Layout::array::<i32>(self.capacity).unwrap();
            unsafe {
                dealloc(self.array, layout);
            }
            self.array = null_mut();
        }
    }

    fn succ(&self, value: usize) -> usize {
        let mut value = value + 1;
        if value == self.capacity {
            value = 0;
        }
        value
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            unsafe {
                *(self.array.add(self.rear)) = x; // use offset to add to the array
            }
        }
    }

    fn front(&self) -> i32 {
        if !self.is_empty() {
            unsafe { *(self.array.add(self.front)) }
        } else {
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = unsafe { *(self.array.add(self.front)) };
            self.size -= 1;
            self.front = self.succ(self.front);
            x
        }
    }
}

fn main() {
    let mut q = Queue::create_queue(12);

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

    q.dispose_queue();
}
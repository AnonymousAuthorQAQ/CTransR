use std::alloc::{self, Layout};
use std::ptr;
use std::io::{self, Write};

type ElementType = i32;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut ElementType,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let layout = Layout::array::<ElementType>(max_elements).map_err(|_| "Out of space!!!")?;
        let array = unsafe { alloc::alloc(layout) };

        if array.is_null() {
            return Err("Out of space!!!");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: capacity - 1,
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

    fn enqueue(&mut self, x: ElementType) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }

        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        unsafe {
            *self.array.add(self.rear) = x;
        }

        Ok(())
    }

    fn front(&self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        unsafe { Ok(*self.array.add(self.front)) }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }

        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;

        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }

        let x = unsafe { *self.array.add(self.front) };
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        Ok(x)
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<ElementType>(self.capacity).unwrap();
            alloc::dealloc(self.array, layout);
        }
    }
}

fn main() {
    let mut queue = Queue::new(12).expect("Failed to create queue");

    // Enqueue numbers from 0 to 9
    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Dequeue and print numbers
    while !queue.is_empty() {
        let front_val = queue.front().expect("Failed to get front value");
        println!("{}", front_val);
        queue.dequeue().expect("Failed to dequeue");
    }

    // Enqueue numbers from 0 to 9 again
    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    // Dequeue and print numbers again
    while !queue.is_empty() {
        let front_val = queue.front().expect("Failed to get front value");
        println!("{}", front_val);
        queue.dequeue().expect("Failed to dequeue");
    }
}
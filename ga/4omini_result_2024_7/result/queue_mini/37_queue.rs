use std::alloc::{self, Layout};
use std::ptr;
use std::io::{self, Write};

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Result<Queue, &'static str> {
        const MIN_QUEUE_SIZE: usize = 5;
        
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let layout = Layout::from_size_align(std::mem::size_of::<i32>() * max_elements, std::mem::align_of::<i32>())
            .map_err(|_| "Out of space!!!")?;

        let array = unsafe { alloc::alloc(layout) };
        if array.is_null() {
            return Err("Out of space!!!");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array,
        })
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }

    fn dispose_queue(&mut self) {
        if !self.array.is_null() {
            unsafe {
                alloc::dealloc(self.array, Layout::from_size_align(std::mem::size_of::<i32>() * self.capacity, std::mem::align_of::<i32>()).unwrap());
            }
            self.array = ptr::null_mut();
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }

        self.size += 1;
        self.rear = self.succ(self.rear);
        unsafe {
            *(self.array.add(self.rear)) = x;
        }
        Ok(())
    }

    fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        unsafe { Ok(*(self.array.add(self.front))) }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        
        let x = unsafe { *(self.array.add(self.front)) };
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(x)
    }
}

fn main() {
    let mut q = Queue::create_queue(12).unwrap();
    
    for i in 0..10 {
        q.enqueue(i).unwrap();
    }

    while !q.is_empty() {
        let value = q.front().unwrap();
        println!("{}", value);
        q.dequeue().unwrap();
    }

    for i in 0..10 {
        q.enqueue(i).unwrap();
    }

    while !q.is_empty() {
        let value = q.front().unwrap();
        println!("{}", value);
        q.dequeue().unwrap();
    }

    q.dispose_queue();
}
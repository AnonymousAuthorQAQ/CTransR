use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32, // Using raw pointer for dynamic array
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, &'static str> {
        if max_elements < 5 {
            return Err("Queue size is too small");
        }

        let layout = Layout::array::<i32>(max_elements).map_err(|_| "Out of space!!!")?;
        let array = unsafe { alloc(layout) };

        if array.is_null() {
            return Err("Out of space!!!");
        }

        Ok(Queue {
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

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            unsafe {
                ptr::write(self.array.add(self.rear), x);
            }
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, &'static str> {
        if !self.is_empty() {
            unsafe { Ok(*self.array.add(self.front)) }
        } else {
            Err("Empty queue")
        }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            let x = unsafe { *self.array.add(self.front) };
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(x)
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
            let layout = Layout::array::<i32>(self.capacity).unwrap();
            unsafe { dealloc(self.array, layout) };
        }
    }
}

fn main() {
    let mut queue = Queue::new(12).expect("Failed to create queue");

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        println!("{}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        println!("{}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }
}
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};
use std::process;

type ElementType = i32;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut ElementType,
}

impl QueueRecord {
    fn new(capacity: usize) -> Self {
        let array = unsafe {
            let layout = Layout::array::<ElementType>(capacity).unwrap();
            alloc(layout) as *mut ElementType
        };
        
        if array.is_null() {
            eprintln!("Out of space!!!");
            process::exit(1);
        }

        QueueRecord {
            capacity,
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
            eprintln!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            unsafe {
                ptr::write(self.array.add(self.rear), x);
            }
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            unsafe { *self.array.add(self.front) }
        } else {
            eprintln!("Empty queue");
            process::exit(1);
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            eprintln!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        } else {
            self.size -= 1;
            let x = unsafe { *self.array.add(self.front) };
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

impl Drop for QueueRecord {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<ElementType>(self.capacity).unwrap();
            dealloc(self.array as *mut u8, layout);
        }
    }
}

fn main() {
    let mut queue = QueueRecord::new(12);

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
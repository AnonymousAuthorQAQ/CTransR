use std::ptr;
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
            let mut alloc = Vec::<ElementType>::with_capacity(capacity);
            let array_ptr = alloc.as_mut_ptr();
            ptr::write_bytes(array_ptr, 0, capacity);
            array_ptr
        };

        Self {
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
            process::exit(1);
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            unsafe { self.array.add(self.rear).write(x) };
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
            process::exit(1);
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x: ElementType;
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        } else {
            self.size -= 1;
            x = unsafe { *self.array.add(self.front) };
            self.front = self.succ(self.front);
        }
        x
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
            Vec::from_raw_parts(self.array, 0, self.capacity);
        }
    }
}

fn main() {
    let mut q = QueueRecord::new(12);

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
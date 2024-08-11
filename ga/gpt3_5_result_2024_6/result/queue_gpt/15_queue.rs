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
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Self {
        if max_elements < 5 {
            eprintln!("Queue size is too small");
            process::exit(1);
        }

        let mut queue = QueueRecord {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: ptr::null_mut(),
        };

        queue.array = Box::into_raw(vec![0; max_elements].into_boxed_slice()) as *mut ElementType;
        queue
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            eprintln!("Full queue");
            process::exit(1);
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            unsafe { *self.array.offset(self.rear as isize) = x };
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            unsafe { *self.array.offset(self.front as isize) }
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
        let x;
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        } else {
            self.size -= 1;
            x = unsafe { *self.array.offset(self.front as isize) };
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

fn main() {
    let mut q = QueueRecord::create_queue(12);

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
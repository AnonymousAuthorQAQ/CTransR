use std::ptr;

type ElementType = i32;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut ElementType,
}

impl Queue {
    fn new(capacity: usize) -> Queue {
        if capacity < 5 {
            panic!("Queue size is too small");
        }
        let array = Box::into_raw(Box::new(vec![0; capacity]));
        Queue {
            capacity,
            front: 0,
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
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            unsafe { *self.array.offset(self.rear as isize) = x };
        }
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        }
        unsafe { *self.array.offset(self.front as isize) }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        }
        let x = unsafe { *self.array.offset(self.front as isize) };
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        x
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.array);
        }
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
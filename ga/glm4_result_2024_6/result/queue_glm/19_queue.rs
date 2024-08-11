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
    fn new(max_elements: usize) -> Self {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }

        let array = unsafe { Box::new_uninit_slice(0..max_elements) };
        let array_ptr = Box::into_raw(array);

        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: array_ptr,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn dispose(&mut self) {
        if !self.array.is_null() {
            unsafe {
                Box::from_raw(self.array);
            }
        }
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            unsafe {
                *self.array.add(self.rear) = x;
            }
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            unsafe { *self.array.add(self.front) }
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

    q.dispose();
}
use std::ptr::null_mut;

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut ElementType,
}

impl Queue {
    fn new(max_elements: usize) -> Option<Self> {
        if max_elements < MIN_QUEUE_SIZE {
            return None;
        }

        let mut array = unsafe { std::alloc::alloc(std::mem::size_of::<ElementType>() * max_elements) };
        if array.is_null() {
            return None;
        }

        Some(Queue {
            capacity: max_elements,
            front: 0,
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

    fn enqueue(&mut self, element: ElementType) {
        if self.is_full() {
            panic!("Queue is full");
        }

        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        unsafe {
            *self.array.add(self.rear) = element;
        }
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            panic!("Queue is empty");
        }

        unsafe { *self.array.add(self.front) }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Queue is empty");
        }

        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let element = self.front();
        self.dequeue();
        element
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.array as *mut _, std::mem::size_of::<ElementType>() * self.capacity);
        }
    }
}

fn main() {
    let mut q = Queue::new(12).unwrap();

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
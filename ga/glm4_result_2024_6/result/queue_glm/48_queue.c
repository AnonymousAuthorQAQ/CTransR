use std::ptr;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

impl Queue {
    fn new(max_elements: usize) -> Self {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }

        let mut array = ptr::null_mut();
        unsafe {
            array = libc::malloc(max_elements * std::mem::size_of::<i32>()) as *mut i32;
        }

        if array.is_null() {
            panic!("Out of space!!!");
        }

        Queue {
            capacity: max_elements,
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

    fn enqueue(&mut self, element: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            unsafe {
                *self.array.offset(self.rear as isize) = element;
            }
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
        }
    }

    fn front(&self) -> i32 {
        if !self.is_empty() {
            unsafe { *self.array.offset(self.front as isize) }
        } else {
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if !self.is_empty() {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        } else {
            panic!("Empty queue");
        }
    }

    fn dispose(&mut self) {
        unsafe {
            libc::free(self.array as *mut libc::c_void);
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
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
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Self {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }

        let array = unsafe { std::alloc::alloc(std::alloc::Layout::array::<ElementType>(max_elements).unwrap()) as *mut ElementType };

        if array.is_null() {
            panic!("Out of space!!!");
        }

        Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array,
        }
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn dispose_queue(&mut self) {
        unsafe {
            std::alloc::dealloc(self.array as *mut u8, std::alloc::Layout::array::<ElementType>(self.capacity).unwrap());
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            unsafe {
                *self.array.offset(self.rear as isize) = x;
            }
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            unsafe {
                *self.array.offset(self.front as isize)
            }
        } else {
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x: ElementType = 0;

        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            let front_element = unsafe { *self.array.offset(self.front as isize) };
            self.front = self.succ(self.front);
            front_element
        }
    }
}

fn main() {
    let mut q = Queue::create_queue(12);

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

    q.dispose_queue();
}
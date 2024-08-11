use std::ptr;
use std::mem;

type ElementType = i32;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut ElementType,
}

fn is_empty(q: &QueueRecord) -> bool {
    q.size == 0
}

fn is_full(q: &QueueRecord) -> bool {
    q.size == q.capacity
}

fn create_queue(max_elements: usize) -> QueueRecord {
    let mut q = QueueRecord {
        capacity: max_elements,
        front: 1,
        rear: 0,
        size: 0,
        array: ptr::null_mut(),
    };

    if max_elements < 5 {
        panic!("Queue size is too small");
    }

    q.array = unsafe {
        let array = libc::malloc(mem::size_of::<ElementType>() * max_elements) as *mut ElementType;
        if array.is_null() {
            panic!("Out of space!!!");
        }
        array
    };

    make_empty(&mut q);

    q
}

fn make_empty(q: &mut QueueRecord) {
    q.size = 0;
    q.front = 1;
    q.rear = 0;
}

fn dispose_queue(q: &mut QueueRecord) {
    if !q.array.is_null() {
        unsafe {
            libc::free(q.array as *mut libc::c_void);
        }
    }
}

fn succ(value: usize, q: &QueueRecord) -> usize {
    if value + 1 == q.capacity {
        0
    } else {
        value + 1
    }
}

fn enqueue(x: ElementType, q: &mut QueueRecord) {
    if is_full(q) {
        panic!("Full queue");
    } else {
        q.size += 1;
        q.rear = succ(q.rear, q);
        unsafe {
            *q.array.offset(q.rear as isize) = x;
        }
    }
}

fn front(q: &QueueRecord) -> ElementType {
    if !is_empty(q) {
        unsafe {
            *q.array.offset(q.front as isize)
        }
    } else {
        panic!("Empty queue");
    }
}

fn dequeue(q: &mut QueueRecord) {
    if is_empty(q) {
        panic!("Empty queue");
    } else {
        q.size -= 1;
        q.front = succ(q.front, q);
    }
}

fn front_and_dequeue(q: &mut QueueRecord) -> ElementType {
    let x: ElementType;

    if is_empty(q) {
        panic!("Empty queue");
    } else {
        q.size -= 1;
        unsafe {
            x = *q.array.offset(q.front as isize);
        }
        q.front = succ(q.front, q);
    }

    x
}

fn main() {
    let mut q = create_queue(12);

    for i in 0..10 {
        enqueue(i, &mut q);
    }

    while !is_empty(&q) {
        println!("{}", front(&q));
        dequeue(&mut q);
    }

    for i in 0..10 {
        enqueue(i, &mut q);
    }

    while !is_empty(&q) {
        println!("{}", front(&q));
        dequeue(&mut q);
    }

    dispose_queue(&mut q);
}
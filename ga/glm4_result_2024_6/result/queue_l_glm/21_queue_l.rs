use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::with_capacity(5);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.pop_front();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..4 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.push_back(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let enqueue_value = read_input();
        queue.push_back(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.pop_front().unwrap());
    }

    // Enqueue elements again
    for i in 0..3 {
        queue.push_back(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue[0]);
        queue.pop_front();
    }
}

fn read_input() -> i32 {
    loop {
        println!("Enter an integer value to enqueue:");
        match std::io::stdin().bytes().next() {
            Some(Ok(b'0'..=b'9')) => return btoi::btoi(&[b]).unwrap(),
            Some(Ok(b'\n')) | None => continue,
            Some(Ok(b)) => println!("Invalid input: {}", b as char),
            Some(Err(e)) => println!("Error: {}", e),
        }
    }
}
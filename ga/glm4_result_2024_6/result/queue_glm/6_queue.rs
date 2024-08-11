use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::with_capacity(12);

    for i in 0..10 {
        queue.push_back(i);
    }

    while let Some(value) = queue.pop_front() {
        println!("{}", value);
    }

    for i in 0..10 {
        queue.push_back(i);
    }

    while let Some(value) = queue.pop_front() {
        println!("{}", value);
    }
}
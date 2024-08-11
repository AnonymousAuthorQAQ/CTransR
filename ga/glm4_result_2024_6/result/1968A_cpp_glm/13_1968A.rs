use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: u64 = input.trim().parse().expect("Please enter a number");

        let mut heap = BinaryHeap::new();
        for mid in 1..x {
            if can(x, mid) {
                heap.push(mid);
            }
        }

        if let Some(max_mid) = heap.pop() {
            println!("{}", max_mid);
        } else {
            println!("No valid mid found");
        }
    }
}

fn can(x: u64, mid: u64) -> bool {
    // Assuming the condition is that gcd(x, mid) + mid should be less than or equal to x
    let sum = gcd(x, mid) + mid;
    sum <= x
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
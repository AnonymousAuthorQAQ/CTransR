use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn can(mid: i64, x: i64) -> bool {
    gcd(mid, x) + mid <= x
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let t = 3; // Replace with actual input
    
    for _ in 0..t {
        let x = 7; // Replace with actual input
        
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || -> i64 {
            let mut l = 1;
            let mut r = x - 1;
            let mut ans = 1;
            
            while l <= r {
                let mid = (l + r) / 2;
                
                if can(mid, x) {
                    ans = mid;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            
            ans
        }).unwrap().send(tx);
        
        println!("Answer: {}", rx.recv().unwrap());
    }
}
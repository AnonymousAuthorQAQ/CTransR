use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        tx.send(input).expect("Failed to send input");
    });

    thread::sleep(Duration::from_secs(1));

    let input = rx.try_recv().expect("Failed to receive input");
    let t = input.trim().parse::<i64>().expect("Invalid input");

    for _ in 0..t {
        let x = input.trim().parse::<i64>().expect("Invalid input");
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;
            if gcd(x, mid) + mid == x {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        println!("{}", ans);
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
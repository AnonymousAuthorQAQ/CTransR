use std::io;

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid > 0
}

// Function to compute GCD using the Euclidean algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    // Set up input handling
    let stdin = io::stdin();
    let input = stdin.lock();

    let mut input_iter = input.lines();
    let t: i64 = input_iter.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let x: i64 = input_iter.next().unwrap().unwrap().trim().parse().unwrap();
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
        println!("{}", ans);
    }
}
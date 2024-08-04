use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i64 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut x_input = String::new();
        io::stdin().read_line(&mut x_input).expect("Failed to read line");
        let x: i64 = x_input.trim().parse().expect("Please enter a number");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;

            if is_coprime(x, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        println!("{}", ans);
    }
}

fn is_coprime(a: i64, b: i64) -> bool {
    gcd(a, b) == 1
}

fn gcd(a: i64, b: i64) -> i64 {
    match Ordering::from((a, b)) {
        Ordering::Less => gcd(b, a),
        Ordering::Equal => a,
        Ordering::Greater => {
            let remainder = a % b;
            if remainder == 0 {
                b
            } else {
                gcd(b, remainder)
            }
        }
    }
}
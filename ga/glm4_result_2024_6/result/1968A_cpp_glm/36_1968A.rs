use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i64 = input.trim().parse().expect("Please type a number");

    for _ in 0..t {
        let mut x_input = String::new();
        io::stdin().read_line(&mut x_input).expect("Failed to read line");
        let x: i64 = x_input.trim().parse().expect("Please type a number");

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

fn can(mid: i64, x: i64) -> bool {
    let gcd = gcd(mid, x);
    gcd + mid <= x
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}
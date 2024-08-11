use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut x_input = String::new();
        io::stdin().read_line(&mut x_input).expect("Failed to read line");
        let x: u64 = x_input.trim().parse().expect("Please enter a number");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = l + (r - l) / 2;

            match (gcd(x, mid) + mid).cmp(&0) {
                Ordering::Greater => {
                    ans = mid;
                    l = mid + 1;
                }
                Ordering::Less => {
                    r = mid - 1;
                }
                Ordering::Equal => {
                    l = mid + 1;
                }
            }
        }

        println!("{}", ans);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
use std::io;
use std::cmp::Ordering;
use std::io::{self, Read};

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

fn can(mid: u64, x: u64) -> bool {
    gcd(x, mid) + mid > mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<u64>().unwrap();
    for _ in 0..t {
        let x = lines.next().unwrap().parse::<u64>().unwrap();
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;
            match can(mid, x).cmp(&true) {
                Ordering::Greater => {
                    ans = mid;
                    l = mid + 1;
                }
                Ordering::Less => {
                    r = mid - 1;
                }
                Ordering::Equal => {
                    ans = mid;
                    l = mid + 1;
                }
            }
        }
        println!("{}", ans);
    }
}
use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: u32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut x_input = String::new();
        io::stdin().read_line(&mut x_input).unwrap();
        let x: u64 = x_input.trim().parse().unwrap();

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = l + (r - l) / 2;
            match gcd(x, mid) {
                Ordering::Equal => {
                    ans = mid;
                    l = mid + 1;
                }
                Ordering::Less => {
                    r = mid - 1;
                }
                Ordering::Greater => {
                    l = mid + 1;
                }
            }
        }

        println!("{}", ans);
    }
}

fn gcd(x: u64, mid: u64) -> Ordering {
    let mut a = x;
    let mut b = mid;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    if a == 1 {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}
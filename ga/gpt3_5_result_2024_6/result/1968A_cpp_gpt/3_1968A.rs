use std::io;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) + mid > 0
}

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: u64 = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut x = String::new();
        io::stdin().read_line(&mut x).unwrap();
        let x: u64 = x.trim().parse().unwrap();

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;
            if can(x, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        
        println!("{}", ans);
    }
}
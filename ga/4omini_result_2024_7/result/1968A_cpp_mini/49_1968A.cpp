use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid > mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();

        let mut l: i64 = 1;
        let mut r: i64 = x - 1;
        let mut ans: i64 = 1;

        while l <= r {
            let mid = (l + r) / 2;
            if can(mid, x) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        
        // Output the result for the current test case
        println!("{}", ans);
    }
}
use std::io;

fn can(mid: i64, x: i64) -> bool {
    (x.gcd(mid) + mid) > 0
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).expect("Failed to read line");
    let t: i64 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let x: i64 = input.trim().parse().expect("Invalid input");
        
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
use std::io;

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

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid > 0
}

fn main() {
    let stdin = io::stdin();
    let mut input_string = String::new();
    
    // Read number of test cases
    stdin.read_line(&mut input_string).unwrap();
    let t: i64 = input_string.trim().parse().unwrap();

    for _ in 0..t {
        input_string.clear();
        // Read value of x for each test case
        stdin.read_line(&mut input_string).unwrap();
        let x: i64 = input_string.trim().parse().unwrap();

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
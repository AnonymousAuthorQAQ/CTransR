fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut t).expect("Failed to read t");
    let t: i32 = t.trim().parse().expect("t is not an integer");

    for _ in 0..t {
        let mut x = 0;
        std::io::stdin().read_line(&mut x).expect("Failed to read x");
        let x: i32 = x.trim().parse().expect("x is not an integer");

        let y = find_y(x);
        println!("{}", y);
    }
}
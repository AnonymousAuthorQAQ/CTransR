fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn find_y(x: i32) -> i32 {
    x - 1
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read input");
    t = t.parse::<i32>().expect("Failed to parse input");

    while t > 0 {
        let mut x = 0;
        std::io::stdin().read_line(&mut String::new()).expect("Failed to read input");
        x = x.parse::<i32>().expect("Failed to parse input");

        let y = find_y(x);
        println!("{}", y);

        t -= 1;
    }
}
fn solve(n: i32) {
    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut t = 1;
    loop {
        println!("Enter the number of test cases: ");
        t = read_int();
        if t == 0 {
            break;
        }
        for _ in 0..t {
            println!("Enter an integer n: ");
            let n = read_int();
            solve(n);
        }
    }
}

fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}
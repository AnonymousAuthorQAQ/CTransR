fn solve(n: usize) {
    let t = n;
    println!("1 1");
    println!("2 1");
    let mut t = t - 2;
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
        t = std::io::stdin().lines().next().unwrap().parse::<usize>().unwrap();
        if t == 0 {
            break;
        }
        println!("Enter {} test cases:", t);
        for _ in 0..t {
            let mut n = String::new();
            std::io::stdin().read_line(&mut n).unwrap();
            let n: usize = n.trim().parse().unwrap();
            solve(n);
        }
    }
}
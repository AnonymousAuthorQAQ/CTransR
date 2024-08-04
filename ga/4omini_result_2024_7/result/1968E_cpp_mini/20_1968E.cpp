use std::io::{self, BufRead};

fn solve(n: usize) {
    let mut output = Vec::new();
    
    output.push("1 1".to_string());
    output.push("2 1".to_string());

    let mut t = n as isize - 2; // Remaining count after first two pairs

    if t > 0 {
        output.push(format!("{} {}", n, n));
        t -= 1;
    }

    for i in 3..=2 + t {
        output.push(format!("1 {}", i));
    }

    for line in output {
        println!("{}", line);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = handle.lines();

    let t: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let n: usize = reader.next().unwrap().unwrap().trim().parse().unwrap();
        solve(n);
    }
}
use std::io;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");
    
    let mut x = vec![0; n];
    for i in 1..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse().expect("Please enter a number");
    }

    let mut a = vec![0; n];
    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=1000000000 {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in a {
        println!("{}", i);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");
    
    for _ in 0..t {
        proc();
    }
}
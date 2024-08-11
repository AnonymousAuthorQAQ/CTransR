use std::io;

const N: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut x = [0; N];
    let mut a = [0; N];

    let n = read_input(&mut x);
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A as usize {
            a[i] = x[i] + (j as i32) * a[i - 1];
            if (i + 1 < n) && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for &val in &a[..n] {
        print!("{} ", val);
    }
    println!();
}

fn read_input(x: &mut [i32; N]) -> usize {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().expect("Invalid input");

    if n >= N {
        panic!("Input size is too large");
    }

    for i in 1..n {
        io::stdin().read_line(&mut line).unwrap();
        x[i] = line.trim().parse().expect("Invalid input");
    }

    n
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().expect("Invalid input");

    for _ in 0..t {
        proc();
    }
}
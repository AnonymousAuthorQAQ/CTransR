const N: usize = 500;
const X: usize = 500;
const A: u64 = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut x: Vec<i32> = vec![0; n];
    for i in 1..n {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse().expect("Invalid input");
    }

    let mut a: Vec<i32> = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j: u64 = 0;
        while j <= A {
            a[i] = x[i] + j as i32 * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && a[i] != a[i - 1]) {
                break;
            }
            j += 1;
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        proc();
    }
}
const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut x: [i32; N] = [0; N];
    let mut a: [i32; N] = [0; N];

    let mut n = 0;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_parts: Vec<&str> = input.trim().split_whitespace().collect();
    n = input_parts[0].parse().unwrap();

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let x_values: Vec<&str> = input.trim().split_whitespace().collect();
    for i in 1..n {
        x[i] = x_values[i - 1].parse().unwrap();
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] != a[i - 1] {
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
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}
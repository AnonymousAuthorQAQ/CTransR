const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut x: [usize; N] = [0; N];
    let mut a: [usize; N] = [0; N];

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut input_x = String::new();
    std::io::stdin().read_line(&mut input_x).unwrap();
    let x_values: Vec<usize> = input_x.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    for i in 1..n {
        x[i] = x_values[i - 1];
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            let next_x = if i + 1 < n { x[i + 1] } else { 1 };
            if next_x < a[i] && a[i] != a[i - 1] {
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
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}
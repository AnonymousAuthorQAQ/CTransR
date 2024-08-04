const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut x: Vec<usize> = vec![0; N];
    let mut a: Vec<usize> = vec![0; N];

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut input_x = String::new();
    std::io::stdin().read_line(&mut input_x).unwrap();
    let input_x: Vec<usize> = input_x.split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    for i in 1..n {
        x[i] = input_x[i - 1];
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && input_x[i] < a[i] && a[i] != a[i - 1]) {
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
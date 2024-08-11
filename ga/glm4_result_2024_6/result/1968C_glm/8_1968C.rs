const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read input for n and x array
    std::io::stdin().read_line(&mut n).expect("Failed to read n");
    for i in 1..n {
        std::io::stdin().read_line(&mut x[i]).expect("Failed to read x[i]");
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && 1 < a[i]) {
                break;
            }
        }
    }

    // Print the a array
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut t).expect("Failed to read t");

    for _ in 0..t {
        proc();
    }
}
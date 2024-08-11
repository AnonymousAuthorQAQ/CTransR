const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    unsafe {
        n = std::io::stdin().lock().read_line().expect("Failed to read line").trim().parse().expect("Invalid input");
        for i in 1..n {
            x[i] = std::io::stdin().lock().read_line().expect("Failed to read line").trim().parse().expect("Invalid input");
        }
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
    let mut t = 0;
    unsafe {
        t = std::io::stdin().lock().read_line().expect("Failed to read line").trim().parse().expect("Invalid input");
    }

    while t > 0 {
        proc();
        t -= 1;
    }
}
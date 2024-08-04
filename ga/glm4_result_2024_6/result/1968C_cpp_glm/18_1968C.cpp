const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read input
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    n = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();
    for i in 1..n {
        x[i] = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<i32>().unwrap();
    }

    // Compute sequence
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        loop {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
            j += 1;
            if j > A {
                break;
            }
        }
    }

    // Print output
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut String::new()).unwrap();
    t = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<i32>().unwrap();
    for _ in 0..t {
        proc();
    }
}
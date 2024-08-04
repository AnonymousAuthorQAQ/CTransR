const N: usize = 500;
const X: usize = 500;
const A: i32 = 1_000_000_000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read n and x
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    n = std::io::stdin().read::<usize>().expect("Failed to read n");
    for i in 1..n {
        x[i] = std::io::stdin().read::<i32>().expect("Failed to read x[i]");
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        while j <= A as usize {
            a[i] = x[i] + j as i32 * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] || i + 1 == n && 1 < a[i] {
                break;
            }
            j += 1;
        }
    }

    // Print a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    t = std::io::stdin().read::<usize>().expect("Failed to read t");
    while t > 0 {
        proc();
        t -= 1;
    }
}
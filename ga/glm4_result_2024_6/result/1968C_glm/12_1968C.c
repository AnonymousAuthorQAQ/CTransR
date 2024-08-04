const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut n;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read the number of elements
    std::io::stdin().read_line(&mut n).expect("Failed to read line");

    // Read the array x
    for i in 1..n {
        std::io::stdin().read_line(&mut x[i]).expect("Failed to read line");
    }

    // Initialize a[0]
    a[0] = x[1] + 1;

    // Calculate the array a
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n ? x[i + 1] : 1) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Print the array a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t;
    std::io::stdin().read_line(&mut t).expect("Failed to read line");

    while t > 0 {
        proc();
        t -= 1;
    }
}
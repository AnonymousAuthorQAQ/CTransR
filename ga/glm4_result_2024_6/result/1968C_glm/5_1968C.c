const N: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read n and n-1 integers into array x
    std::io::stdin().read_line(&mut n).expect("Failed to read n");
    for i in 1..n {
        std::io::stdin().read_line(&mut x[i]).expect("Failed to read x[i]");
    }

    // Initialize a[0]
    a[0] = x[1] + 1;

    // Calculate the values of array a
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n ? x[i + 1] : 1) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Print the elements of array a
    for i in 0..n {
        println!("{}", a[i]);
    }
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut t).expect("Failed to read t");
    for _ in 0..t {
        proc();
    }
}
const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut n;
    loop {
        // Read the size of the sequence
        if let Ok(input) = std::io::stdin().read_line() {
            if let Ok(n_value) = input.trim().parse::<usize>() {
                n = n_value;
                if n > 1 {
                    break;
                }
            }
        }
    }

    // Initialize the x array and read the sequence
    let mut x = [0; N];
    for i in 1..n {
        if let Ok(input) = std::io::stdin().read_line() {
            if let Ok(x_value) = input.trim().parse::<usize>() {
                x[i] = x_value;
            }
        }
    }

    // Calculate the a array
    let mut a = [0; N];
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        loop {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i == n - 1 && 1 < a[i]) {
                if a[i] != a[i - 1] {
                    break;
                }
            }
            if j >= A {
                break;
            }
            j += 1;
        }
    }

    // Print the a array
    for i in 0..n {
        println!("{}", a[i]);
    }
}

fn main() {
    let mut t;
    loop {
        // Read the number of test cases
        if let Ok(input) = std::io::stdin().read_line() {
            if let Ok(t_value) = input.trim().parse::<usize>() {
                t = t_value;
                break;
            }
        }
    }

    // Call proc() for each test case
    for _ in 0..t {
        proc();
    }
}
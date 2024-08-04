const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read n
    n = std::io::stdin().read_line().unwrap().trim().parse::<usize>().unwrap();

    // Read sequence
    for i in 1..n {
        x[i] = std::io::stdin().read_line().unwrap().trim().parse::<usize>().unwrap();
    }

    // Initialize a[0]
    a[0] = x[1] + 1;

    // Calculate the rest of the sequence
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if let Some(&next_x) = x.get(i + 1) {
                if next_x < a[i] && a[i] != a[i - 1] {
                    break;
                }
            } else {
                if 1 < a[i] && a[i] != a[i - 1] {
                    break;
                }
            }
        }
    }

    // Print the sequence
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line().unwrap().trim().parse::<usize>().unwrap();

    while t > 0 {
        proc();
        t -= 1;
    }
}
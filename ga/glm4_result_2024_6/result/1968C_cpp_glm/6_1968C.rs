const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read n
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    n = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();

    // Read x[i] for i = 1 to n-1
    for i in 1..n {
        x[i] = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Print a[0] to a[n-1]
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;

    // Read t
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    t = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();

    while t > 0 {
        proc();
        t -= 1;
    }
}
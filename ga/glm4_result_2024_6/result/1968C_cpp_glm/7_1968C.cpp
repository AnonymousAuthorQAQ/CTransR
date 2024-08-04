const N: usize = 500;
const X: usize = 500;
const A: u64 = 1_000_000_000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read n
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    n = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();

    // Read x array
    for i in 1..n {
        std::io::stdin().read_line(&mut String::new()).unwrap();
        x[i] = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();
    }

    // Initialize a[0]
    a[0] = x[1] + 1;

    // Calculate a[i]
    for i in 1..n {
        let mut j = 0;
        loop {
            a[i] = x[i] + (j as u64 * a[i - 1]) as usize;
            let next_x = if i + 1 < n { x[i + 1] } else { 1 };
            if next_x < a[i] && a[i] != a[i - 1] {
                break;
            }
            j += 1;
        }
    }

    // Print a array
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;

    // Read t
    std::io::stdin().read_line(&mut String::new()).unwrap();
    t = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();

    while t > 0 {
        proc();
        t -= 1;
    }
}
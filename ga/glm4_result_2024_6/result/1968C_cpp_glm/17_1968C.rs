const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read the number of elements
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    n = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<usize>().unwrap();

    // Read the array elements
    for i in 1..n {
        std::io::stdin().read_line(&mut String::new()).unwrap();
        x[i] = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<i32>().unwrap();
    }

    // Initialize the first element of a
    a[0] = x[1] + 1;

    // Compute the array a based on x
    for i in 1..n {
        let mut j = 0;
        while j <= A as usize {
            a[i] = x[i] + j as i32 * a[i - 1];
            if (i + 1 < n) && (x[i + 1] < a[i]) && (a[i] != a[i - 1]) {
                break;
            }
            j += 1;
        }
    }

    // Print the array a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut String::new()).unwrap();
    t = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse::<i32>().unwrap();

    while t > 0 {
        proc();
        t -= 1;
    }
}
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: usize = input.trim().parse().expect("Invalid input");
        let mut x = Vec::with_capacity(n);
        x.push(0);

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        for num in input.trim().split_whitespace() {
            x.push(num.parse().expect("Invalid input"));
        }

        let mut a = Vec::with_capacity(n);
        a.push(x[1] + 1);

        for i in 1..n {
            let mut j = 0;
            loop {
                a.push(x[i] + j * a[i - 1]);
                if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && 1 < a[i]) {
                    break;
                }
                j += 1;
            }
        }

        for num in a {
            print!("{} ", num);
        }
        println!();
    }
}
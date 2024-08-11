use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i32 = input.trim().parse().expect("Please type a number!");

        let mut x = vec![0; n as usize];
        for i in 1..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            x[i as usize] = input.trim().parse().expect("Please type a number!");
        }

        let mut a = vec![0; n as usize];
        a[0] = x[1] + 1;

        for i in 1..n {
            let mut j = 0;
            while j <= 1000000000 {
                a[i as usize] = x[i as usize] + j * a[(i - 1) as usize];
                if (i + 1 < n) && (x[(i + 1) as usize] < a[i as usize]) && (a[i as usize] != a[(i - 1) as usize]) {
                    break;
                }
                j += 1;
            }
        }

        for i in 0..n {
            print!("{} ", a[i as usize]);
        }
        println!();
    }
}
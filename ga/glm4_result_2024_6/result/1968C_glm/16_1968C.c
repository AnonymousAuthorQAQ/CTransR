use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        let mut x = vec![0; n as usize];
        let mut a = vec![0; n as usize];

        for i in 1..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            x[i as usize] = input.trim().parse().unwrap();
        }

        a[0] = x[1] + 1;

        for i in 1..n {
            for j in 0..=1000000000 {
                a[i as usize] = x[i as usize] + j * a[(i - 1) as usize];
                if (i + 1 < n && x[(i + 1) as usize] < a[i as usize]) || (i + 1 == n && 1 < a[i as usize]) {
                    break;
                }
            }
        }

        for i in 0..n {
            print!("{} ", a[i as usize]);
        }
        println!();
    }
}
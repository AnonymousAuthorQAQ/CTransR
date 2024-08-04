use std::cmp;

const N: usize = 200_000;

fn main() {
    let mut input = String::new();

    // Read the number of test cases
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    // Process each test case
    for _ in 0..t {
        input.clear();
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = 0;
        let mut b = 0;

        std::io::stdin().read_line(&mut input).unwrap();
        let params: Vec<usize> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        
        let (n, k, initial_a, initial_b) = (params[0], params[1], params[2], params[3]);
        a = initial_a;
        b = initial_b;

        let mut arr = vec![0; n];
        let mut aa = vec![0; n];

        // Read arr values
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        for (i, value) in input.trim().split_whitespace().enumerate() {
            arr[i] = value.parse().unwrap();
        }

        // Read aa values
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        for (i, value) in input.trim().split_whitespace().enumerate() {
            aa[i] = value.parse().unwrap();
        }

        let m = cmp::min(n, k);
        let mut a_index = a;
        let mut b_index = b;

        for x in 1..=m {
            if a_index > 0 {
                p1 = cmp::max(p1, (aa[a_index - 1] as i64) * (k - x + 1) + p1_);
                p1_ += aa[a_index - 1] as i64;
                a_index = arr[a_index - 1];
            }
            if b_index > 0 {
                p2 = cmp::max(p2, (aa[b_index - 1] as i64) * (k - x + 1) + p2_);
                p2_ += aa[b_index - 1] as i64;
                b_index = arr[b_index - 1];
            }
        }

        // Output the result of the match
        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}
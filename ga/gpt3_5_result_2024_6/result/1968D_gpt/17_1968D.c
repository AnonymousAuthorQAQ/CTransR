use std::cmp;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let split_input: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = split_input[0];
        let k = split_input[1];
        let mut a = split_input[2];
        let mut b = split_input[3];
        let m = cmp::min(n, k);
        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);
        let mut arr: Vec<usize> = Vec::with_capacity(n + 1);
        let mut aa: Vec<usize> = Vec::with_capacity(n + 1);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let arr_input: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        arr.push(0);
        arr.extend_from_slice(&arr_input);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa_input: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        aa.push(0);
        aa.extend_from_slice(&aa_input);

        for x in 1..=m {
            p1 = cmp::max(p1, (aa[a] * (k - x + 1)) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = cmp::max(p2, (aa[b] * (k - x + 1)) as i64 + p2_);
            p2_ += aa[b];
            b = arr[b];
        }

        let winner = match p1.cmp(&p2) {
            cmp::Ordering::Greater => "Bodya",
            cmp::Ordering::Less => "Sasha",
            cmp::Ordering::Equal => "Draw",
        };

        println!("{}", winner);
    }
}
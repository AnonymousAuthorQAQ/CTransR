use std::cmp;

const N: usize = 200000;

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: i32 = t.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.split_whitespace().map(|x| x.parse().unwrap());
        let n = nums.next().unwrap();
        let k = nums.next().unwrap();
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        let m = cmp::min(n, k);
        let mut a_ = a;
        let mut b_ = b;
        let mut arr: Vec<i32> = vec![0; n + 1];
        let mut aa: Vec<i32> = vec![0; n + 1];

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        arr[1..].copy_from_slice(&nums);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        aa[1..].copy_from_slice(&nums);

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for x in 1..=m {
            p1 = cmp::max(p1, aa[a as usize] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = cmp::max(p2, aa[b as usize] as i64 * (k - x + 1) as i64 + p2_);
            p2_ += aa[b as usize];
            b = arr[b as usize];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}
use std::cmp;

const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut t;
    let mut p1;
    let mut p2;
    let mut p1_;
    let mut p2_;
    let mut i;
    let mut i_;
    let mut j;
    let mut j_;
    let n;
    let m;
    
    let mut arr = [0; N + 1];
    let mut aa = [0; N + 1];
    
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    t = input.trim().parse::<i32>().unwrap();
    
    for _ in 0..t {
        p1 = 0;
        p2 = 0;
        p1_ = 0;
        p2_ = 0;
        i = -1;
        i_ = -1;
        j = -1;
        j_ = -1;
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut values = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        n = values.next().unwrap();
        let k = values.next().unwrap();
        let a = values.next().unwrap();
        let b = values.next().unwrap();
        let mut a_ = a;
        let mut b_ = b;
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for x in 1..=n {
            arr[x] = input.split_whitespace().nth(x-1).unwrap().parse::<i32>().unwrap();
        }
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for x in 1..=n {
            aa[x] = input.split_whitespace().nth(x-1).unwrap().parse::<i32>().unwrap();
        }
        
        for x in 1..=cmp::min(n, k) {
            p1 = maxl(p1, (aa[a] * (k - x + 1) as i64) + p1_);
            p1_ += aa[a];
            a = arr[a as usize];
            p2 = maxl(p2, (aa[b] * (k - x + 1) as i64) + p2_);
            p2_ += aa[b];
            b = arr[b as usize];
        }
        
        let result = if p1 > p2 {
            "Bodya\n"
        } else if p1 < p2 {
            "Sasha\n"
        } else {
            "Draw\n"
        };
        
        print!("{}", result);
    }
}
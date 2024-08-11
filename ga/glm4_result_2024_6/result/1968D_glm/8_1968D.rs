use std::io::{self, Read};

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let t: i32 = lines[0].parse().unwrap();

    for i in 1..=t {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = 0;
        let mut b = 0;

        let mut arr: Vec<i32> = Vec::new();
        let mut aa: Vec<i32> = Vec::new();

        for (j, line) in lines[i * 5..(i * 5) + 5].iter().enumerate() {
            match j {
                0 => {
                    let mut iter = line.split_whitespace();
                    let n: i32 = iter.next().unwrap().parse().unwrap();
                    let k: i32 = iter.next().unwrap().parse().unwrap();
                    a = iter.next().unwrap().parse().unwrap();
                    b = iter.next().unwrap().parse().unwrap();
                    arr.resize(n as usize, 0);
                    aa.resize(n as usize, 0);
                }
                1 => {
                    let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                    arr.copy_from_slice(&nums);
                }
                2 => {
                    let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                    aa.copy_from_slice(&nums);
                }
                _ => {}
            }
        }

        for x in 1..=min(arr.len() as i32, a) {
            p1 = maxl(p1, (aa[a as usize] as i64) * (k - x + 1) as i64 + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];
        }

        for x in 1..=min(arr.len() as i32, b) {
            p2 = maxl(p2, (aa[b as usize] as i64) * (k - x + 1) as i64 + p2_);
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
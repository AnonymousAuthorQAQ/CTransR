use std::io::{self, BufRead};

const N: usize = 200000;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<String> = handle.lines().map(|line| line.unwrap()).collect();
    
    let mut line_index = 0;

    let t: usize = lines[line_index].parse().unwrap();
    line_index += 1;

    for _ in 0..t {
        let params: Vec<usize> = lines[line_index]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        line_index += 1;

        let n = params[0];
        let k = params[1];
        let mut a = params[2] - 1; // 0-based index
        let mut b = params[3] - 1; // 0-based index
        let m = n.min(k);

        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];
        
        for x in 1..=n {
            arr[x] = lines[line_index].split_whitespace().nth(x - 1).unwrap().parse().unwrap();
        }
        line_index += 1;
        
        for x in 1..=n {
            aa[x] = lines[line_index].split_whitespace().nth(x - 1).unwrap().parse().unwrap();
        }
        line_index += 1;

        let mut p1 = 0i64;
        let mut p2 = 0i64;
        let mut p1_ = 0i64;
        let mut p2_ = 0i64;

        for x in 1..=m {
            p1 = p1.max((aa[a] as i64) * (k - x as i64 + 1) + p1_);
            p1_ += aa[a] as i64;
            a = (arr[a + 1] - 1) as usize; // Move to the next index (1-based to 0-based)
            
            p2 = p2.max((aa[b] as i64) * (k - x as i64 + 1) + p2_);
            p2_ += aa[b] as i64;
            b = (arr[b + 1] - 1) as usize; // Move to the next index (1-based to 0-based)
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
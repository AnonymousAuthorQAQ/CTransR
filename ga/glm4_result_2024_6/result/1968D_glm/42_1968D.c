use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    
    let t = {
        stdin.read_line(&mut line).unwrap();
        line.trim().parse::<i32>().unwrap()
    };
    
    while t > 0 {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let k = parts.next().unwrap().parse::<i32>().unwrap();
        let a = parts.next().unwrap().parse::<i32>().unwrap();
        let b = parts.next().unwrap().parse::<i32>().unwrap();
        
        let mut arr = vec![0; n as usize + 1];
        let mut aa = vec![0; n as usize + 1];
        
        for i in 1..=n {
            stdin.read_line(&mut line).unwrap();
            arr[i as usize] = line.trim().parse::<i32>().unwrap();
        }
        
        for i in 1..=n {
            stdin.read_line(&mut line).unwrap();
            aa[i as usize] = line.trim().parse::<i32>().unwrap();
        }
        
        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;
        
        for x in 1..=k {
            p1 = p1.max(p1_ + aa[a as usize]);
            p1_ += aa[a as usize];
            a = arr[a as usize];
            
            p2 = p2.max(p2_ + aa[b as usize]);
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
        
        t -= 1;
    }
}
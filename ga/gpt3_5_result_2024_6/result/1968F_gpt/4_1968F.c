use std::io::{self, BufRead};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Random {
        Random { z: 1 }
    }
    
    fn rand(&mut self) -> i32 {
        self.z = self.z.wrapping_mul(3);
        (self.z >> 1) as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        handle.read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        
        input.clear();
        handle.read_line(&mut input).unwrap();
        let aa: Vec<u32> = input.trim().split_whitespace()
                                        .map(|x| x.parse().unwrap())
                                        .collect();
        
        let mut aa_xor: Vec<u32> = vec![0; n + 1];
        for i in 1..=n {
            aa_xor[i] = aa_xor[i - 1] ^ aa[i - 1];
        }
        
        let mut ii: Vec<usize> = (0..=n).collect();
        
        sort(&mut ii, &aa_xor, 0, n + 1);
        
        for _ in 0..q {
            input.clear();
            handle.read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();
            
            let idx_left = search(&ii, &aa_xor, r, l);
            let idx_right = search(&ii, &aa_xor, l, r);
            
            if aa_xor[l] == aa_xor[r] || ii[idx_left + 1] < ii[idx_right] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

fn sort(ii: &mut Vec<usize>, aa: &Vec<u32>, l: usize, r: usize) {
    if l < r {
        let i_ = l + Random::new().rand() as usize % (r - l);
        let mut i = l;
        let mut j = l;
        let mut k = r;
        
        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
            } else {
                ii[j] - i_
            };
            
            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                i += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        
        sort(ii, aa, l, i);
        sort(ii, aa, k, r);
    }
}

fn search(ii: &Vec<usize>, aa: &Vec<u32>, a: usize, i: usize) -> usize {
    let mut lower = -1;
    let mut upper = ii.len() as i32;
    
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        
        if aa[ii[h as usize]] < a as u32 || aa[ii[h as usize]] == a as u32 && ii[h as usize] < i {
            lower = h;
        } else {
            upper = h;
        }
    }
    
    lower as usize
}
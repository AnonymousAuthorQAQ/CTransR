use std::io::{self, Read};
use std::time::SystemTime;

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now();
        let duration = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let z = (duration.as_secs() ^ duration.subsec_micros() as u64) as u32 | 1;
        Random { z }
    }
    
    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;
    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (rand.rand() % (r - l) as u32) as usize];
        
        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - i_ as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                l += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, i, l, rand);
        l = k;
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32], n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let nums: Vec<usize> = lines.next().unwrap().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1])
        };

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();
        
        for i in 1..=n {
            let x: u32 = lines.next().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ x;
        }
        
        let mut rand = Random::new();
        sort(&mut ii, &aa, 0, n + 1, &mut rand);
        
        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let nums: Vec<usize> = lines.next().unwrap().split_whitespace()
                    .map(|s| s.parse().unwrap()).collect();
                (nums[0], nums[1])
            };
            let l = l - 1; // Convert to 0-based index
            let res = if aa[l] == aa[r] || ii[search(aa[r], l, &ii, &aa, n)] < ii[search(aa[l], r, &ii, &aa, n)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", res);
        }
    }
}
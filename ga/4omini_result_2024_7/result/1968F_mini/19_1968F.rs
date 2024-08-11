use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let sec = start.as_secs() as u32;
        let usec = start.subsec_micros() as u32;
        let z = sec ^ usec | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rand.rand() % (r - l as u32) as usize)];
        let mut j = j;
        let mut k = k;
        
        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as isize - aa[i_] as isize
            } else {
                ii[j] as isize - ii[i] as isize
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                j += 1;
                l += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }

        sort(ii, aa, l, l + k - j, rand);
        l = k;
    }
}

fn search(ii: &[usize], aa: &[u32], a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = ii.len() as isize;

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
    let mut input = stdin.lock().lines();

    let t: usize = input.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut params = input.next().unwrap().unwrap();
        let mut params = params.split_whitespace();

        let n: usize = params.next().unwrap().parse().unwrap();
        let q: usize = params.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();
        let mut rand = Random::new();

        for i in 1..=n {
            let mut x: u32 = input.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ x;
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            let mut l_r = input.next().unwrap().unwrap();
            let mut l_r = l_r.split_whitespace();
            let mut l: usize = l_r.next().unwrap().parse().unwrap();
            let mut r: usize = l_r.next().unwrap().parse().unwrap();

            l -= 1; // To convert to zero-indexed
            if aa[l] == aa[r] || ii[(search(&ii, &aa, aa[r], l) + 1) as usize] < ii[search(&ii, &aa, aa[l], r) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
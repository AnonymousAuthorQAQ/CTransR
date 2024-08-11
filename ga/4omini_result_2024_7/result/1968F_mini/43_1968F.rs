use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (start.as_secs() ^ start.subsec_micros() as u64) as u32 | 1;
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
        let i_ = ii[l + (rand.rand() % (r - l) as usize)];

        let mut j = j;
        let mut k = k;

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

        sort(ii, aa, l, l + (k - l), rand);
        l = k;
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize], n: usize) -> isize {
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
    let mut input = stdin.lock();
    let mut buffer = String::new();
    
    let mut rand = Random::new();
    let mut t: usize = 0;

    input.read_line(&mut buffer).unwrap();
    t = buffer.trim().parse().unwrap();
    buffer.clear();

    while t > 0 {
        t -= 1;
        
        let mut n: usize = 0;
        let mut q: usize = 0;

        input.read_line(&mut buffer).unwrap();
        let mut parts = buffer.split_whitespace();
        n = parts.next().unwrap().parse().unwrap();
        q = parts.next().unwrap().parse().unwrap();
        buffer.clear();

        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            input.read_line(&mut buffer).unwrap();
            aa[i] = buffer.trim().parse().unwrap();
            aa[i] ^= aa[i - 1];
            buffer.clear();
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        while q > 0 {
            q -= 1;
            let mut l: usize = 0;
            let mut r: usize = 0;

            input.read_line(&mut buffer).unwrap();
            let mut parts = buffer.split_whitespace();
            l = parts.next().unwrap().parse().unwrap();
            r = parts.next().unwrap().parse().unwrap();
            buffer.clear();
            l -= 1; // convert to zero-based index

            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii, n) as usize + 1] < ii[search(aa[l], r, &aa, &ii, n) as usize] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}
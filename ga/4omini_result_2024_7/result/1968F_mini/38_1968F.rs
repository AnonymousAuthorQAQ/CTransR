use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seconds = start.as_secs() as u32;
        let microseconds = start.subsec_millis() / 1000;
        Self { z: seconds ^ microseconds | 1 }
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
        let mut k = r;
        let i_ = ii[l + (rand.rand() as usize) % (r - l)];
        
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
        sort(ii, aa, l, i, rand);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize, n: usize) -> isize {
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
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    let mut rand = Random::new();
    
    for _ in 0..t {
        let (n, q) = {
            let nums: Vec<usize> = lines.next().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        };

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        for i in 1..=n {
            aa[i] = {
                let x: u32 = lines.next().unwrap().parse().unwrap();
                aa[i - 1] ^ x
            };
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            let (l, r) = {
                let vals: Vec<usize> = lines.next().unwrap().split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                (vals[0] - 1, vals[1])  // l is made 0-indexed here
            };
            let result = if aa[l] == aa[r] || 
                ii[search(&aa, &ii, aa[r], l, n) as usize + 1] < ii[search(&aa, &ii, aa[l], r, n)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}
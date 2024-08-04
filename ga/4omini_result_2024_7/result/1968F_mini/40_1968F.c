use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, BufRead};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).unwrap();
        let z = (duration.as_secs() ^ duration.subsec_millis() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(aa: &mut [u32], ii: &mut [usize], l: usize, r: usize) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand() % (r - l)];

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - ii[i] as i32
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
        sort(aa, ii, l, i);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize) -> isize {
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
    let mut lines = stdin.lock().lines();

    let mut rng = Random::new();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let parts: Vec<usize> = lines.next().unwrap().unwrap()
                .split_whitespace().map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        };

        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            let mut value: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = value ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut aa, &mut ii, 0, n + 1);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let parts: Vec<usize> = lines.next().unwrap().unwrap()
                    .split_whitespace().map(|s| s.parse().unwrap()).collect();
                (parts[0] - 1, parts[1])
            };

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) as usize + 1] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}
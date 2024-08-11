use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct RNG {
    z: u32,
}

impl RNG {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seconds = start.as_secs() as u32;
        let microseconds = start.subsec_micros() as u32;
        let z = (seconds ^ microseconds) | 1; // Ensure z is non-zero
        RNG { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &Vec<u32>, l: usize, r: usize, rng: &mut RNG) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let random_index = l + (rng.rand() % (r - l) as u32) as usize;
        let pivot_val = aa[ii[random_index]];

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[ii[j]] != pivot_val {
                aa[ii[j]] as isize - pivot_val as isize
            } else {
                (ii[j] as isize) - (ii[i] as isize)
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, l, i, rng);
        l = k;
    }
}

fn search(aa: &Vec<u32>, ii: &Vec<usize>, a: u32, n: usize, i: usize) -> isize {
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
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut rng = RNG::new();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let parts: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        };

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        for i in 1..=n {
            let x: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ x; // Prefix XOR
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let parts: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                    .map(|x| x.parse().unwrap()).collect();
                (parts[0] - 1, parts[1]) // adjust for 0-indexing
            };

            let answer = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], n, l) as usize + 1] < ii[search(&aa, &ii, aa[l], n, r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", answer);
        }
    }
}
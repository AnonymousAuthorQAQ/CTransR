use std::io::{self, BufRead};
use std::time::SystemTime;

static N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> SpecialNumGenerator {
        let start = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        // Use seconds XOR microseconds to initialize
        Random {
            z: start.as_secs() as u32 ^ start.subsec_micros() ^ 1,
        }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(arr: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rand_gen: &mut Random) {
    let mut left = l;
    let mut right = l;
    let mut k = r;

    while left < k {
        let pivot_index = l + (rand_gen.rand() % (r - l) as u32) as usize;
        let pivot = aa[arr[pivot_index]];
        let mut i = left;

        while right < k {
            let comp = if aa[arr[right]] != pivot {
                aa[arr[right]] as i64 - pivot as i64
            } else {
                arr[right] as i64 - arr[pivot_index] as i64
            };

            match comp {
                0 => right += 1,
                _ if comp < 0 => {
                    arr.swap(i, right);
                    i += 1;
                    right += 1;
                }
                _ => {
                    k -= 1;
                    arr.swap(right, k);
                }
            }
        }
        sort(arr, aa, l, i, rand_gen);
        left = k;
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize], n: usize) -> isize {
    let mut lower = -1;
    let mut upper: isize = n as isize + 1;

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

    let mut t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    while t > 0 {
        t -= 1;

        let n_q: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let n = n_q[0];
        let q = n_q[1];

        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            let mut temp: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = temp ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        let mut rand_gen = Random::new();
        sort(&mut ii, &aa, 0, n + 1, &mut rand_gen);

        for _ in 0..q {
            let lr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let l = lr[0] - 1;
            let r = lr[1];

            let answer = if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii, n)] < ii[search(aa[l], r, &aa, &ii, n)] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", answer);
        }
    }
}
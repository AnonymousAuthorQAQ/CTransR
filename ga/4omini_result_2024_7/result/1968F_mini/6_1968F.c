use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        // Initialize random seed based on the current time
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seed = start.as_secs() ^ (start.subsec_micros() as u64) as u64 | 1;
        Self { z: seed as u32 }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(aa: &[u32], ii: &mut Vec<usize>, l: usize, r: usize, random: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (random.rand() as usize % (r - l))];

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
                l += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(aa, ii, l, k, random);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize) -> usize {
    let mut lower = usize::MAX;
    let mut upper = ii.len();

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h]] < a || aa[ii[h]] == a && ii[h] < i {
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

    let mut first_line = String::new();
    input.read_line(&mut first_line).unwrap();
    let t: usize = first_line.trim().parse().unwrap();

    for _ in 0..t {
        let mut second_line = String::new();
        input.read_line(&mut second_line).unwrap();
        let mut iter = second_line.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut place = String::new();
            input.read_line(&mut place).unwrap();
            let value: u32 = place.trim().parse().unwrap();
            aa[i] = value ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        let mut random = Random::new();
        sort(&aa, &mut ii, 0, n + 1, &mut random);

        for _ in 0..q {
            let mut range_input = String::new();
            input.read_line(&mut range_input).unwrap();
            let mut range_iter = range_input.split_whitespace();
            let l: usize = range_iter.next().unwrap().parse().unwrap();
            let r: usize = range_iter.next().unwrap().parse().unwrap();
            
            // l is changed to 0-based index
            let l = l - 1;

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) + 1] < ii[search(&aa, &ii, aa[l], r)] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}
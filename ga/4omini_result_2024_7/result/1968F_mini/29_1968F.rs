use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn rand_seed() -> u32 {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).unwrap();
    let seed = (duration.as_secs() as u32) ^ (duration.subsec_millis() as u32) | 1;
    seed
}

struct RNG {
    z: u32,
}

impl RNG {
    fn new(seed: u32) -> Self {
        Self { z: seed }
    }

    fn rand(&mut self) -> u32 {
        // Update the state and return a new random number.
        self.z = self.z.checked_mul(3).unwrap_or(0);
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rng: &mut RNG) {
    let mut left = l;
    let mut right = l;

    while left < r {
        let i = left;
        let k = r;
        let pivot_index = lo_index = l + (rng.rand() % ((r - l) as u32)) as usize;
        let pivot_value = aa[ii[pivot_index]];
        
        while right < k {
            let current_value = aa[ii[right]];
            let cmp_result = match current_value.cmp(&pivot_value) {
                std::cmp::Ordering::Equal => ii[right].cmp(&ii[i]),
                other => other,
            };
            
            if cmp_result == std::cmp::Ordering::Equal {
                right += 1;
            } else if cmp_result == std::cmp::Ordering::Less {
                ii.swap(i, right);
                left += 1;
                right += 1;
            } else {
                k -= 1;
                ii.swap(right, k);
            }
        }

        sort(ii, aa, l, left, rng);
        left = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize) -> isize {
    let mut lower = -1isize;
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
    let mut rng = RNG::new(rand_seed());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
        let n = parts[0];
        let q = parts[1];

        let mut aa = vec![0u32; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let values: Vec<u32> = input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

        // Prepare the aa array with prefix XOR
        aa[0] = values[0];
        for i in 1..n {
            aa[i] = aa[i - 1] ^ values[i];
        }

        // Sort based on prefix XOR
        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<usize> = input.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
            let l = parts[0] - 1;
            let r = parts[1] - 1;

            let result = if aa[l] == aa[r] || ii[(search(&aa, &ii, aa[r], l) + 1) as usize] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}
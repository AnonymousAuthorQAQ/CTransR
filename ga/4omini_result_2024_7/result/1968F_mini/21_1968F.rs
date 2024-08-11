use std::io::{self, BufRead};
use std::time::SystemTime;
use rand::Rng;

const N: usize = 200_000;

struct State {
    z: u32,
}

impl State {
    fn new() -> Self {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        // Initialize 'z' using a combination of seconds and microseconds, ensuring z is at least 1
        let z = (time.as_secs() ^ time.subsec_micros() as u64) as u32 | 1;
        State { z }
    }

    fn rand_(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, state: &mut State) {
    let mut l = l;
    let mut r = r;
    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l] + (state.rand_() as usize % (r - l));

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i64 - aa[i_] as i64
            } else {
                ii[j] as i64 - ii[i] as i64
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
        sort(ii, aa, l, i, state);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, n: usize) -> usize {
    let mut lower = isize::MIN;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < lower as usize) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower as usize
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut rng_state = State::new();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = lines.next().unwrap().unwrap();
            let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        };

        let mut aa = vec![0u32; n + 1]; // Array 1-indexed, index 0 is unused
        let mut ii: Vec<usize> = (0..=n).collect(); // Index array

        for i in 1..=n {
            aa[i] = lines.next().unwrap().unwrap().trim().parse::<u32>().unwrap();
            aa[i] ^= aa[i - 1]; // Cumulative XOR
        }
        
        // Sorting indices based on the values in aa
        sort(&mut ii, &aa, 0, n + 1, &mut rng_state);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = lines.next().unwrap().unwrap();
                let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            };
            let l = l - 1; // Convert to 0-indexed
            let aa_l = aa[l];
            let aa_r = aa[r];

            let search_lower_r = search(&aa, &ii, aa_r, n);
            let search_lower_l = search(&aa, &ii, aa_l, n);
            if aa_l == aa_r || ii[search_lower_r + 1] < ii[search_lower_l] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
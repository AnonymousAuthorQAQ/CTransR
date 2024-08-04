use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

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
        self.z *= 3;  // Updating the state
        self.z >> 1  // Return the random number
    }
}

fn sort(aa: &[u32], ii: &mut [usize], l: usize, r: usize, rng: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rng.rand() % (r - l) as usize)];

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as isize - aa[i_] as isize
            } else {
                ii[j] as isize - i_ as isize
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
        sort(aa, ii, l, k, rng);
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
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut rng = Random::new();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = lines.next().unwrap().unwrap();
            let nums: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            (nums[0], nums[1])
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let num: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ num;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&aa, &mut ii, 0, n + 1, &mut rng);
        
        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = lines.next().unwrap().unwrap();
                let nums: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                (nums[0] - 1, nums[1])
            };
            let answer = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) as usize + 1] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", answer);
        }
    }
}
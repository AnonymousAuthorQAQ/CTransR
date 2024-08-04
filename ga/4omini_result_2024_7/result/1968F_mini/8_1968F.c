use std::io;
use std::time::{Duration, Instant};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = Instant::now();
        let seconds = now.elapsed().as_secs() as u32;
        let nanos = now.elapsed().subsec_nanos() / 1_000_000; // convert nanoseconds to milliseconds
        let z = seconds ^ nanos | 1; // use the current time to seed the random generator
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
        let i_ = ii[l + (rand.rand() as usize % (r - l))];

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
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }

        sort(ii, aa, l, j, rand);
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let mut rand = Random::new();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let params: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = params[0];
        let q = params[1];

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<u32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        for i in 1..=n {
            aa[i] = nums[i - 1] ^ aa[i - 1];
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let lr: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = lr[0] - 1;
            let r = lr[1];

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) as usize + 1] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}
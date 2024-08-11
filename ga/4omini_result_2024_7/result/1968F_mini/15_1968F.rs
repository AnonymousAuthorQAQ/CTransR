use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (now.as_secs() ^ now.subsec_millis() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &Vec<u32>, l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rand.rand() % (r - l) as usize)];
        let mut j = j;

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
        sort(ii, aa, l, i, rand);
        l = k;
    }
}

fn search(a: u32, i: usize, aa: &Vec<u32>, ii: &Vec<usize>, n: usize) -> isize {
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
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut rand = Random::new();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let nums: Vec<usize> = lines.next().unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        };

        let mut aa: Vec<u32> = vec![0; n + 1];
        for i in 1..=n {
            let value: u32 = lines.next().unwrap().trim().parse().unwrap();
            aa[i] = aa[i - 1] ^ value;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let nums: Vec<usize> = lines.next().unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                (nums[0] - 1, nums[1]) // convert to 0-indexed
            };

            let answer = if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii, n)] < ii[search(aa[l], r, &aa, &ii, n)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", answer);
        }
    }
}
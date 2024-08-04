use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let z = (since_epoch.as_secs() ^ since_epoch.subsec_micros() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(arr: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rng: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = arr[l + (rng.rand() % (r - l) as u32) as usize];

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[arr[j]] != aa[i_] {
                aa[arr[j]] as i32 - aa[i_] as i32
            } else {
                arr[j] as i32 - i_ as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                arr.swap(i, j);
                i += 1;
                j += 1;
            } else {
                k -= 1;
                arr.swap(j, k);
            }
        }
        sort(arr, aa, l, i, rng);
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
    let mut rng = Random::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<u32> = input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        
        for i in 1..=n {
            aa[i] = nums[i - 1] ^ aa[i - 1];
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace();
            let l: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse::<usize>().unwrap();

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l, n)] < ii[search(&aa, &ii, aa[l], r, n)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}
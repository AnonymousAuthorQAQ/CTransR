use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now();
        let epoch = now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
        let seconds = epoch.as_secs() as u32;
        let microseconds = epoch.subsec_micros() as u32;

        let z = (seconds ^ microseconds) | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, random: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (random.rand() % (r - l) as usize)];

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
        sort(ii, aa, l, i, random);
        l = k;
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize], n: usize) -> isize {
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
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let mut random = Random::new();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa = vec![0u32; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for i in 1..=n {
            let value: u32 = input.trim().split_whitespace().nth(i - 1).unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ value;
        }

        sort(&mut ii, &aa, 0, n + 1, &mut random);

        for _ in 0..q {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse::<usize>().unwrap();

            if aa[l] == aa[r] || ii[(search(aa[r], l, &aa, &ii, n) + 1) as usize] < ii[search(aa[l], r, &aa, &ii, n) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
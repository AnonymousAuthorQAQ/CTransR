use std::io::{self, BufRead};

const N: usize = 200000;

struct Rand {
    z: u32,
}

impl Rand {
    fn new() -> Rand {
        let mut z = 0;
        let tv = chrono::Utc::now().timestamp();
        z = (tv as u32) ^ 1;
        Rand { z }
    }

    fn rand(&mut self) -> i32 {
        self.z = self.z.wrapping_mul(3) >> 1;
        (self.z >> 1) as i32
    }
}

fn sort(ii: &mut [usize], aa: &[i32], l: usize, r: usize, rand_gen: &mut Rand) {
    if l < r {
        let i_ = l + rand_gen.rand() as usize % (r - l);
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
            } else {
                ii[j] as i32 - i_ as i32
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
        sort(ii, aa, l, i, rand_gen);
        sort(ii, aa, k, r, rand_gen);
    }
}

fn search(aa: &[i32], ii: &[usize], a: i32, i: usize) -> i32 {
    let mut lower = -1;
    let mut upper = (aa.len() + 1) as i32;

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
    let mut handle = stdin.lock();

    let mut input_line = String::new();
    handle.read_line(&mut input_line).unwrap();
    let t: i32 = input_line.trim().parse().unwrap();

    for _ in 0..t {
        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        let mut parts = input_line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        let aa: Vec<i32> = input_line.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut Rand::new());

        for _ in 0..q {
            input_line.clear();
            handle.read_line(&mut input_line).unwrap();
            let mut parts = input_line.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();

            let result = if aa[l - 1] == aa[r] || ii[search(&aa, &ii, aa[r], l - 1) as usize + 1] < ii[search(&aa, &ii, aa[l - 1], r)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}
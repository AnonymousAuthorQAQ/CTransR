use std::io::{self, BufRead};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let z = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        Random { z: z | 1 }
    }

    fn rand(&mut self) -> i32 {
        self.z = self.z.wrapping_mul(3) >> 1;
        self.z as i32
    }
}

fn sort(ii: &mut [usize], aa: &[i32], l: usize, r: usize, rand_gen: &mut Random) {
    if l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[l + (rand_gen.rand() as usize) % (r - l)];

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

fn search(aa: &[i32], ii: &[usize], a: i32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = aa.len() as isize + 1;

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
    let mut lines = stdin.lock().lines();
    
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut rand_gen = Random::new();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        aa[0] = 0;

        let mut ii: Vec<usize> = (0..=n).collect();

        for i in 1..=n {
            let num: i32 = iter.next().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ num;
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rand_gen);

        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse().unwrap();

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) as usize + 1] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}
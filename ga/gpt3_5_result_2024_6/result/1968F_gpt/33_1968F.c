use std::io::{self, BufRead};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Random {
        Random { z: 0 }
    }

    fn rand(&mut self) -> i32 {
        self.z = self.z.wrapping_mul(3) >> 1;
        (self.z as i32)
    }

    fn srand(&mut self) {
        let tv = chrono::offset::Local::now().timestamp_subsec_nanos() as u32;
        self.z = tv ^ 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    let mut z = Random::new();
    
    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[l + z.rand() as usize % (r - l)];
        let mut tmp: usize;

        while j < k {
            let c = if aa[ii[j]] != i_ { aa[ii[j]] as i32 - i_ as i32 } else { ii[j] as i32 - i_ as i32 };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = ii[i];
                ii[i] = ii[j];
                ii[j] = tmp;
                i += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = ii[j];
                ii[j] = ii[k];
                ii[k] = tmp;
            }
        }
        sort(ii, l, i, aa);
        l = k;
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> i32 {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as i32;

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
        let mut iter = input_line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        let aa: Vec<u32> = input_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            input_line.clear();
            handle.read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            let l = l - 1;
            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &ii, &aa) as usize + 1] < ii[search(aa[l], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}
use std::io::{self, BufRead};

const N: usize = 200_000;

struct Rand {
    z: u32,
}

impl Rand {
    fn new() -> Self {
        Rand { z: 0 }
    }

    fn rand(&mut self) -> i32 {
        self.z *= 3;
        self.z >> 1
    }

    fn srand(&mut self) {
        let time = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as u32,
            Err(_) => 1,
        };
        self.z = time ^ 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize, rand: &mut Rand) {
    if l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[l + (rand.rand() as usize) % (r - l)];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ { aa[ii[j]] - i_ } else { ii[j] - i_ };

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
        sort(aa, ii, l, i, rand);
        sort(aa, ii, k, r, rand);
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
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = iterator.next().unwrap().trim().parse().unwrap();
    let mut rand = Rand::new();

    for _ in 0..t {
        let line: String = iterator.next().unwrap();
        let mut values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let q = values.next().unwrap();
        let mut aa = vec![0; n as usize + 1];

        for i in 1..=n {
            aa[i] = values.next().unwrap() ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n as usize).collect();
        sort(&mut aa, &mut ii, 0, n as usize + 1, &mut rand);
        
        for _ in 0..q {
            let query_line: String = iterator.next().unwrap();
            let mut query_values = query_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
            let l = query_values.next().unwrap() - 1;
            let r = query_values.next().unwrap();

            let result = if aa[l] == aa[r] || ii[(search(&aa, &ii, aa[r], l) + 1) as usize] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}
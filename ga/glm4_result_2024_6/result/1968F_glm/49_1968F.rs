use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe {
        Z = (Z * 3) >> 1;
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().expect("Time went backwards");
    unsafe {
        Z = (duration.as_secs() as u32 ^ duration.subsec_nanos() as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        let mut tmp;

        while j < k {
            let c = if crate::aa[ii[j]] != crate::aa[i_] {
                crate::aa[ii[j]] - crate::aa[i_]
            } else {
                ii[j] - i_
            };

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
        sort(ii, l, i);
        l = k;
    }
}

fn search(a: i32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = crate::n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if crate::aa[crate::ii[h as usize]] < a
            || (crate::aa[crate::ii[h as usize]] == a && crate::ii[h as usize] < i)
        {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    let t = crate::read_int();

    crate::srand_();
    for _ in 0..t {
        let (n, q) = crate::read_two_ints();
        crate::n = n;

        for i in 1..=n {
            crate::aa[i] = crate::read_int();
            crate::aa[i] ^= crate::aa[i - 1];
        }

        for i in 0..=n {
            crate::ii[i] = i;
        }

        crate::sort(&mut crate::ii, 0, n + 1);

        for _ in 0..q {
            let (l, r) = crate::read_two_ints();
            l -= 1;

            if crate::aa[l] == crate::aa[r]
                || crate::ii[crate::search(crate::aa[r], l) as usize + 1] < crate::ii[crate::search(crate::aa[l], r)]
            {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

mod crate {
    use std::io;

    pub static mut aa: [i32; 200_001] = [0; 200_001];
    pub static mut n: usize = 0;
    pub static mut ii: [usize; 200_001] = [0; 200_001];

    pub fn read_int() -> i32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse::<i32>().unwrap()
    }

    pub fn read_two_ints() -> (i32, i32) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    }
}
use std::io;
use std::cmp::Ordering;

const N: usize = 200000;
static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe {
        Z = Z.wrapping_mul(3);
        Z >>= 1;
        Z
    }
}

fn srand_() {
    unsafe {
        let tv = std::time::SystemTime::now();
        let since_epoch = tv.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
        Z = (since_epoch.as_secs() as u32 ^ since_epoch.subsec_nanos() as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if r - l > 1 {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        while j < k {
            let c = if crate::aa::aa[ii[j]] != crate::aa::aa[i_] {
                crate::aa::aa[ii[j]] - crate::aa::aa[i_]
            } else {
                ii[j] - i_
            };
            match c.cmp(&0) {
                Ordering::Less => {
                    ii.swap(i, j);
                    i += 1;
                    j += 1;
                }
                Ordering::Equal => {
                    j += 1;
                }
                Ordering::Greater => {
                    k -= 1;
                    ii.swap(j, k);
                }
            }
        }
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = crate::aa::n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if crate::aa::aa[crate::aa::ii[h as usize]] < a || (crate::aa::aa[crate::aa::ii[h as usize]] == a && crate::aa::ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }

    lower
}

fn main() {
    srand_();
    let t = read_int();

    for _ in 0..t {
        let (n, q) = read_tuple();
        let mut aa = [0; N + 1];
        let mut ii = [0; N + 1];

        for i in 1..=n {
            aa[i] = read_int();
            aa[i] ^= aa[i - 1];
        }

        for i in 0..=n {
            ii[i] = i;
        }

        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let (l, r) = read_tuple();
            let l = l - 1;

            if aa[l] == aa[r] || crate::aa::ii[search(aa[r], l) as usize + 1] < crate::aa::ii[search(aa[l], r) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

fn read_int() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Failed to parse integer")
}

fn read_tuple() -> (u32, u32) {
    let input = read_int();
    (input >> 16, input & 0xFFFF)
}
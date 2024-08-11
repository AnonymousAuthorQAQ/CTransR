use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = (Z.wrapping_mul(3) >> 1) | 1;
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");
    let seconds = since_the_epoch.as_secs() as u32;
    let nanoseconds = since_the_epoch.subsec_nanos() as u32;
    unsafe {
        Z = seconds ^ nanoseconds;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        while j < k {
            let c = if i_ < j {
                std::cmp::Ordering::Less
            } else if i_ > j {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            };
            match c {
                std::cmp::Ordering::Less => {
                    ii.swap(i, j);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    j += 1;
                }
                std::cmp::Ordering::Greater => {
                    k -= 1;
                    ii.swap(j, k);
                }
            }
        }
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> usize {
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
    lower as usize
}

fn main() {
    srand_();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let q: usize = values.next().unwrap().parse().unwrap();
        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();
        for i in 1..=n {
            aa[i] = values.next().unwrap().parse::<u32>().unwrap() ^ aa[i - 1];
        }
        sort(&mut ii, 0, n + 1);
        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut values = input.trim().split_whitespace();
            let l: usize = values.next().unwrap().parse().unwrap();
            let r: usize = values.next().unwrap().parse().unwrap();
            println!(
                if aa[l - 1] == aa[r - 1] || ii[search(aa[r - 1], l - 1, &aa, &ii) + 1] < ii[search(
                    aa[l - 1],
                    r - 1,
                    &aa,
                    &ii
                )] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}
use std::io::{self, Read};

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand() {
    let mut tv = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        libc::gettimeofday(&mut tv, std::ptr::null_mut());
        let Z = (tv.tv_sec as u32) ^ (tv.tv_usec as u32) | 1;
        unsafe {
            std::mem::transmute::<&mut u32, &mut u32>(&mut Z);
        }
    }
}

fn sort(aa: &mut [i32], n: usize, ii: &mut [usize]) {
    fn sort_helper(aa: &mut [i32], l: usize, r: usize, ii: &mut [usize]) {
        if l < r {
            let mut i = l;
            let mut j = l;
            let mut k = r;
            let i_ = ii[l + rand() as usize % (r - l)];
            while j < k {
                let c = if aa[ii[j]] != aa[i_] {
                    aa[ii[j]] - aa[i_]
                } else {
                    ii[j] - i_
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
            sort_helper(aa, l, i, ii);
            sort_helper(aa, k, r, ii);
        }
    }
    sort_helper(aa, 0, n, ii);
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = (n + 1) as isize;
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
    srand();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let q = parts.next().unwrap().parse::<usize>().unwrap();
        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = parts.next().unwrap().parse::<i32>().unwrap();
            aa[i] ^= aa[i - 1];
        }
        let mut ii = (0..=n).collect::<Vec<_>>();
        sort(&mut aa, n, &mut ii);
        for _ in 0..q {
            let mut parts = lines.next().unwrap().split_whitespace();
            let l = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r = parts.next().unwrap().parse::<usize>().unwrap();
            println!(
                "{}",
                if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii) as usize + 1] < ii[search(aa[l], r, &aa, &ii)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}
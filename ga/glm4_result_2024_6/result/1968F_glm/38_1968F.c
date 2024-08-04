use std::io;

fn rand() -> usize {
    static mut Z: usize = 0;
    unsafe {
        Z = (Z * 3) >> 1;
        Z
    }
}

fn srand() {
    unsafe {
        let mut tv: libc::timeval = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        libc::gettimeofday(&mut tv, std::ptr::null_mut());
        Z = tv.tv_sec as usize ^ tv.tv_usec as usize | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize]) {
    fn partition(l: usize, r: usize, aa: &mut [i32], ii: &mut [usize]) -> usize {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + rand() % (r - l)];
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
        i
    }

    fn quicksort(l: usize, r: usize, aa: &mut [i32], ii: &mut [usize]) {
        if l < r {
            let p = partition(l, r, aa, ii);
            quicksort(l, p, aa, ii);
            quicksort(p, r, aa, ii);
        }
    }

    quicksort(0, aa.len(), aa, ii);
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = ii.len() + 1;
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h]] < a || (aa[ii[h]] == a && ii[h] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    unsafe {
        srand();
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        let mut aa: Vec<i32> = Vec::with_capacity(n + 1);
        aa.push(0);
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa.push(input.trim().parse().unwrap());
        }
        for i in 1..=n {
            aa[i] ^= aa[i - 1];
        }
        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut aa, &mut ii);
        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();
            println!(
                "{}",
                if aa[l - 1] == aa[r - 1] || ii[search(aa[r - 1], l - 1, &aa, &ii) + 1] < ii[search(aa[l - 1], r - 1, &aa, &ii)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}
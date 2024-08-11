use std::io;
use std::time::SystemTime;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(System::UNIX_EPOCH).expect("Time went backwards");
    let seconds = since_the_epoch.as_secs();
    let nanoseconds = since_the_epoch.subsec_nanos();
    unsafe {
        Z = (seconds as u32) ^ (nanoseconds as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        while j < k {
            let c = if aa[i_] != aa[ii[j]] {
                aa[i_] - aa[ii[j]]
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
        sort(ii, l, i);
        l = k;
    }
}

fn search(a: i32, i: usize) -> isize {
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
    srand_();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa: Vec<i32> = Vec::with_capacity(n + 1);
        aa.push(0);
        for _ in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa.push(input.trim().parse::<i32>().unwrap());
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();
            let a_l = aa[l];
            let a_r = aa[r];

            let l_search = search(a_l, l);
            let r_search = search(a_r, r);

            if a_l == a_r || (l_search != -1 && r_search != -1 && ii[l_search as usize] < ii[r_search as usize]) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
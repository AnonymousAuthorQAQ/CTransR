use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand_() {
    unsafe {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Z = (time.as_secs() as u32) ^ (time.as_micros() as u32) | 1;
    }
}

fn sort(ii: &mut Vec<usize>, l: usize, r: usize, aa: &Vec<u32>) {
    if l < r {
        let i_ = ii[l + (rand_() % (r - l))];
        let mut i = l;
        let mut j = l;
        let mut k = r;
        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] - i_ };
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
        sort(ii, l, i, aa);
        sort(ii, k, r, aa);
    }
}

fn search(a: u32, i: usize, ii: &Vec<usize>, aa: &Vec<u32>, n: usize) -> usize {
    let mut lower = -1;
    let mut upper = n + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h]] < a || (aa[ii[h]] == a && ii[h] < i) {
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
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let q: usize = values.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let mut aa: Vec<u32> = vec![0; n + 1];
        let mut ii: Vec<usize> = vec![0; n + 1];

        for i in 1..=n {
            aa[i] = values.next().unwrap().parse().unwrap() ^ aa[i - 1];
        }

        for i in 0..=n {
            ii[i] = i;
        }

        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut values = input.trim().split_whitespace();
            let l: usize = values.next().unwrap().parse().unwrap() - 1;
            let r: usize = values.next().unwrap().parse().unwrap();

            let left = search(aa[r], l, &ii, &aa, n) + 1;
            let right = search(aa[l], r, &ii, &aa, n);

            if aa[l] == aa[r] || ii[left] < ii[right] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
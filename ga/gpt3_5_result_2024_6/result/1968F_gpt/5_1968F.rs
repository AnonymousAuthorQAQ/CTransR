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
    let tv = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as u32;
    unsafe {
        Z = tv ^ 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    let mut l = l;
    let mut r = r;
    while l < r {
        let i_ = aa[ii[l + rand_() as usize % (r - l)]];
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ { aa[ii[j]] - i_ } else { ii[j] - l };

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

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> isize {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as isize;

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
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let q: usize = values.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut values = input.split_whitespace();
            let l: usize = values.next().unwrap().parse().unwrap();
            let r: usize = values.next().unwrap().parse().unwrap();

            if aa[l] == aa[r] || ii[(search(aa[r], l, &ii, &aa) + 1) as usize] < ii[search(aa[l], r, &ii, &aa) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
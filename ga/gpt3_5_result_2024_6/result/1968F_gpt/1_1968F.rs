use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand() {
    let tv = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    unsafe {
        Z = (tv.as_secs() ^ tv.subsec_micros()) as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {
    if l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + rand() as usize % (r - l)]];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ { aa[ii[j]] - i_ } else { ii[j] as i32 - i_ };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = ii[i]; ii[i] = ii[j]; ii[j] = tmp;
                i += 1; j += 1;
            } else {
                k -= 1;
                tmp = ii[j]; ii[j] = ii[k]; ii[k] = tmp;
            }
        }
        sort(ii, l, i, aa);
        sort(ii, k, r, aa);
    }
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> i32 {
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
    srand();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: i32 = iter.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            if aa[l] == aa[r] || ii[(search(aa[r], l, &aa, &ii) + 1) as usize] < ii[search(aa[l], r, &aa, &ii) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
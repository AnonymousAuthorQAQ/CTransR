use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = (Z.wrapping_mul(3) >> 1) ^ (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u32);
        Z |= 1;
        Z
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
            let c = if ii[j] != i_ { ii[j] - i_ } else { ii[j] - i_ };

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

fn search(a: usize, i: usize, aa: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = aa.len() as isize;

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let q = nums.next().unwrap();
        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();

        for i in 1..=n {
            aa[i] = nums.next().unwrap();
            aa[i] ^= aa[i - 1];
        }

        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut query = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
            let l = query.next().unwrap() - 1;
            let r = query.next().unwrap();

            let l = if aa[l] == aa[r] || ii[search(aa[r], l, &aa) + 1] < ii[search(aa[l], r, &aa)] {
                println!("YES");
            } else {
                println!("NO");
            };
        }
    }
}
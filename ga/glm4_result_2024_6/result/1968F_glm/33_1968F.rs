use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|s| s.parse().unwrap());
        let n: usize = nums.next().unwrap();
        let q: usize = nums.next().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();

        for i in 1..=n {
            aa[i] = nums.next().unwrap() ^ aa[i - 1];
        }

        sort(&mut ii, &aa, 0, n);

        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut nums = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
            let l: usize = nums.next().unwrap();
            let r: usize = nums.next().unwrap();

            println!("{}", if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) + 1] < ii[search(&aa, &ii, aa[l], r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn sort(ii: &mut [usize], aa: &[i32], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_(r - l)];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] - i_ };

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

        sort(ii, aa, l, i);
        l = k;
    }
}

fn search(aa: &[i32], ii: &[usize], a: i32, i: usize) -> usize {
    let mut lower = -1;
    let mut upper = ii.len() as i32;

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

fn rand_(n: usize) -> usize {
    let now = SystemTime::now();
    let seed = now.elapsed().unwrap().subsec_nanos() as usize;
    let mut z = seed | 1;
    z = (z.wrapping_mul(3)) >> 1;
    z % n
}
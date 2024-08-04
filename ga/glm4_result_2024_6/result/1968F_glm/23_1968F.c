use std::io::{self, Read};

const N: usize = 200000;

fn rand_() -> u32 {
    use rand::Rng;
    rand::thread_rng().gen()
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i_ = l + rand_() as usize % (r - l);
        let i_ = ii[i_];
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let mut tmp;
        while j < k {
            if aa[ii[j]] < aa[i_] || (aa[ii[j]] == aa[i_] && ii[j] < i_) {
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
        sort(ii, k, r);
    }
}

fn search(a: i32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = N as isize + 1;
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
        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut parts = lines.next().unwrap().split_whitespace();
            let l = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r = parts.next().unwrap().parse::<usize>().unwrap();
            println!("{}", if aa[l] == aa[r] || ii[search(aa[r], l) as usize + 1] < ii[search(aa[l], r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}
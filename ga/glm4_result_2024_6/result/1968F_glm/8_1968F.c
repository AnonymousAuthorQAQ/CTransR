use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut input = String::new();
    let mut stdout = io::BufWriter::new(stdout);

    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut line = input.trim().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let q: usize = line.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();

        for i in 1..=n {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            aa[i] = input.trim().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        sort(&mut ii, &aa, 0, n + 1);

        for _ in 0..q {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            let mut line = input.trim().split_whitespace();
            let l: usize = line.next().unwrap().parse().unwrap();
            let r: usize = line.next().unwrap().parse().unwrap();

            let l = l - 1;
            let result = aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) + 1] < ii[search(&aa, &ii, aa[l], r)];

            writeln!(stdout, "{}", if result { "YES" } else { "NO" }).unwrap();
        }
    }
}

fn sort(ii: &mut [usize], aa: &[i32], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand::rand_range(l, r)];
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

mod rand {
    use std::time::SystemTime;

    pub fn rand_range(l: usize, r: usize) -> usize {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;

        let mut z = seed ^ (seed >> 21) ^ (seed >> 42);
        z = z.wrapping_add(0x7F4A7C15);
        (z % (r - l + 1) + l) as usize
    }
}
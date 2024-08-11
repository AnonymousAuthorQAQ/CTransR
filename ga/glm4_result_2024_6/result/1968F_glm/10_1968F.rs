use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while t > 0 {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

        let mut aa: Vec<i32> = (0..=n).map(|_| lines.next().unwrap().unwrap().parse().unwrap()).collect();
        for i in 1..=n {
            aa[i] ^= aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();

        fn sort(aa: &Vec<i32>, ii: &mut Vec<usize>, l: usize, r: usize) {
            if l < r {
                let i_ = ii[l + rand_(r - l)];
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
                sort(aa, ii, l, i);
                sort(aa, ii, k, r);
            }
        }

        sort(&aa, &mut ii, 0, n + 1);

        while q > 0 {
            let l: usize = lines.next().unwrap().unwrap().parse().unwrap() - 1;
            let r: usize = lines.next().unwrap().unwrap().parse().unwrap();

            let mut lower = -1;
            let mut upper = n + 1;

            while upper - lower > 1 {
                let h = (lower + upper) / 2;
                if aa[ii[h]] < aa[r] || (aa[ii[h]] == aa[r] && ii[h] < l) {
                    lower = h;
                } else {
                    upper = h;
                }
            }

            println!("{}", if aa[l] == aa[r] || (upper < n && ii[upper + 1] < ii[lower + 1]) { "YES" } else { "NO" });
            q -= 1;
        }

        t -= 1;
    }
}

fn rand_(n: usize) -> usize {
    use rand::Rng;
    rand::thread_rng().gen_range(0..n)
}

fn srand_() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen();
}
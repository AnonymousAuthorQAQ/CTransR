use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let q = parts.next().unwrap().parse::<usize>().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = parts.next().unwrap().parse::<usize>().unwrap();
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, &aa, 0, n + 1);

        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let l = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r = parts.next().unwrap().parse::<usize>().unwrap() - 1;

            println!("{}", if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) + 1] < ii[search(&aa, &ii, aa[l], r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn sort(ii: &mut [usize], aa: &[i32], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand::thread_rng().gen_range(l..r)];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
            } else {
                ii[j] - i_
            };

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
        sort(ii, aa, k, r);
    }
}

fn search(aa: &[i32], ii: &[usize], a: i32, i: usize) -> usize {
    let mut lower = -1;
    let mut upper = ii.len();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let aa = [0, 1, 2, 2, 3, 4, 5];
        let ii = [0, 1, 2, 3, 4, 5, 6];
        assert_eq!(search(&aa, &ii, 2, 1), 2);
        assert_eq!(search(&aa, &ii, 3, 1), 4);
        assert_eq!(search(&aa, &ii, 5, 1), 6);
    }
}
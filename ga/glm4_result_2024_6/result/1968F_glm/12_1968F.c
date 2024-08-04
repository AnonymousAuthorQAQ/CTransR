use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    for _ in 0..t {
        let n_q = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let (n, q) = (n_q.next().unwrap(), n_q.next().unwrap());

        let mut aa = Vec::with_capacity(n as usize + 1);
        aa.push(0);
        for _ in 0..n {
            aa.push(lines.next().unwrap().unwrap().parse::<i32>().unwrap());
        }

        for i in 1..=n {
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<_>>();

        sort(&mut ii, &aa, 0, n as usize + 1);

        for _ in 0..q {
            let l_r = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap());
            let (l, r) = (l_r.next().unwrap() - 1, l_r.next().unwrap());

            println!("{}", if aa[l] == aa[r] || ii[search(&aa, &ii, l, r) + 1] < ii[search(&aa, &ii, r, l)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn sort(ii: &mut [i32], aa: &[i32], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = *ii.get(l + rand_() % (r - l)).unwrap();
        let mut tmp;

        while j < k {
            let c = if aa[*ii.get(j).unwrap()] != aa[i_] {
                aa[*ii.get(j).unwrap()] - aa[i_]
            } else {
                *ii.get(j).unwrap() - i_
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = *ii.get(i).unwrap();
                *ii.get_mut(i).unwrap() = *ii.get(j).unwrap();
                *ii.get_mut(j).unwrap() = tmp;
                i += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = *ii.get(j).unwrap();
                *ii.get_mut(j).unwrap() = *ii.get(k).unwrap();
                *ii.get_mut(k).unwrap() = tmp;
            }
        }
        sort(ii, aa, l, i);
        l = k;
    }
}

fn search(a: &i32, i: i32, aa: &[i32], ii: &[i32]) -> i32 {
    let mut lower = -1;
    let mut upper = aa.len() as i32;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[*ii.get(h as usize).unwrap()] < *a || (aa[*ii.get(h as usize).unwrap()] == *a && *ii.get(h as usize).unwrap() < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn rand_() -> i32 {
    static mut Z: i32 = 0;
    unsafe {
        Z = (Z * 3) >> 1;
        Z
    }
}

fn srand_() {
    unsafe {
        Z = rand::random();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let aa = vec![0, 1, 2, 3, 4, 5];
        let ii = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(search(&2, 2, &aa, &ii), 2);
        assert_eq!(search(&3, 3, &aa, &ii), 3);
        assert_eq!(search(&4, 3, &aa, &ii), 4);
    }
}
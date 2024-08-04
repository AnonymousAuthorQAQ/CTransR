use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &mut Vec<i32>, n: usize, st: &mut Vec<i32>) {
	let mut n_ = 1;
	while n_ < n {
		n_ <<= 1;
	}
	st.clear();
	st.resize(n_ * 2, 0);
	for i in 0..n {
		st[n_ + i] = aa[i];
	}
	for i in (1..n_).rev() {
		st[i] = max(st[i << 1], st[(i << 1) + 1]);
	}
}

fn query(l: usize, a: i32, st: &mut Vec<i32>, n_: usize) -> usize {
	let mut r = n_ - 1;
    let mut l = l;

    while l <= r {
        let mut l_ = l;
		if l & 1 == 1 {
			if st[l] >= a {
				while l_ < n_ {
					l = if st[l << 1] >= a { l << 1 } else { (l << 1) + 1 };
					l_ = st[l];
				}
				return l - n_;
			}
			l += 1;
		}
		l >>= 1;
		r >>= 1;
	}
	n_
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let t: i32 = input.trim().parse().unwrap();

	for _ in 0..t {
		input.clear();
		std::io::stdin().read_line(&mut input).unwrap();
		let v: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let n = v[0];
        let kl = v[1];
        let kr = v[2];
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let cc = input.trim();

		let mut zz = vec![0; n];
		for i in 1..n {
			let (mut l, mut r) = (0, 0);
			if i + zz[i - l] < r {
				zz[i] = zz[i - l];
			} else {
				l = i;
				r = cmp::max(r, i);
				while r < n && cc.chars().nth(r) == cc.chars().nth(r - l) {
					r += 1;
				}
				zz[i] = r - l;
			}
		}

        let mut st = vec![];
        build(&mut zz, n, &mut st);

		let mut ans = vec![0; n + 1];
		for p in 1..=n {
			let mut k = 0;
			let mut i = 0;

			loop {
				i = query(i + p, p as i32, &mut st, n_);
				if i < n {
					k += 1;
				} else {
					break;
				}
			}
			ans[k] = cmp::max(ans[k], p);
		}

		for k in (1..n).rev() {
			ans[k] = cmp::max(ans[k], ans[k + 1]);
		}

		for k in kl..=kr {
			print!("{} ", ans[k]);
		}
		println!();
	}
}
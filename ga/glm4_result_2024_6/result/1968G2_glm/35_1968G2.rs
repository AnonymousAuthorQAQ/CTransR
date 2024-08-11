const N: usize = 200000;
const N_: usize = (1 << 18); // N_ = pow2(ceil(log2(N)))

struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(n: usize) -> SegmentTree {
        let mut tree = vec![0; 2 * N_];
        SegmentTree { tree, n }
    }

    fn build(&mut self, arr: &[i32]) {
        for i in 0..self.n {
            self.tree[N_ + i] = arr[i];
        }
        for i in (1..N_).rev() {
            self.tree[i] = self.tree[i << 1].max(self.tree[i << 1 | 1]);
        }
    }

    fn query(&self, l: usize, r: usize, a: i32) -> i32 {
        let mut l = l + N_;
        let mut r = r + N_;
        let mut ans = i32::MIN;

        while l <= r {
            if l & 1 == 1 {
                if self.tree[l] >= a {
                    ans = ans.max(self.tree[l]);
                    while l < N_ {
                        l = if self.tree[l << 1] >= a { l << 1 } else { l << 1 | 1 };
                    }
                    return ans;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        i32::MIN
    }
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    t = std::io::stdin().read_line(&mut String::new()).unwrap().trim().parse().unwrap();

    while t > 0 {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n = iter.next().unwrap().parse().unwrap();
        let kl = iter.next().unwrap().parse().unwrap();
        let kr = iter.next().unwrap().parse().unwrap();
        let cc = iter.next().unwrap();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = std::cmp::max(r, i);
                while r < n && &cc[r..r + (r - l)] == &cc[l..r] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut st = SegmentTree::new(n);
        st.build(&zz);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = st.query(i, n - 1, p) as usize;
                k += 1;
            }
            ans[k] = std::cmp::max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = std::cmp::max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
        t -= 1;
    }
}
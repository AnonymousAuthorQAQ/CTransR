const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

fn maxl(a: i64, b: i64) -> i64 {
    a.max(b)
}

fn main() {
    let mut t = 0;
    std::io::stdin().read_line(&mut t).expect("Failed to read t");

    for _ in 0..t {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut j = -1;
        let mut a = 0;
        let mut b = 0;
        let mut n = 0;
        let mut k = 0;
        let mut arr = [0; N + 1];
        let mut aa = [0; N + 1];

        std::io::stdin().read_line(&mut n).expect("Failed to read n");
        std::io::stdin().read_line(&mut k).expect("Failed to read k");
        std::io::stdin().read_line(&mut a).expect("Failed to read a");
        std::io::stdin().read_line(&mut b).expect("Failed to read b");

        for x in 1..=n {
            std::io::stdin().read_line(&mut arr[x]).expect("Failed to read arr[x]");
        }

        for x in 1..=n {
            std::io::stdin().read_line(&mut aa[x]).expect("Failed to read aa[x]");
        }

        for x in 1..=k {
            p1 = maxl(p1, 1i64 * aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, 1i64 * aa[b] * (k - x + 1) + p2_);
            p2_ += aa[b];
            b = arr[b];
        }

        println!("{}{}{}", if p1 > p2 { "Bodya" } else { if p1 < p2 { "Sasha" } else { "Draw" } });
    }
}
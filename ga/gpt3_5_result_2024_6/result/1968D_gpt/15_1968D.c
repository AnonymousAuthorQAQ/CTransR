fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let k = values.next().unwrap();
        let mut a = values.next().unwrap();
        let mut b = values.next().unwrap();

        let m = n.min(k);
        let mut arr = Vec::with_capacity(n as usize);
        let mut aa = Vec::with_capacity(n as usize);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        arr.extend(input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()));
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        aa.extend(input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()));
        
        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);
        
        for x in 1..=m {
            p1 = p1.max((aa[a as usize - 1] * (k - x + 1)) as i64 + p1_);
            p1_ += aa[a as usize - 1];
            a = arr[a as usize - 1];

            p2 = p2.max((aa[b as usize - 1] * (k - x + 1)) as i64 + p2_);
            p2_ += aa[b as usize - 1];
            b = arr[b as usize - 1];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}
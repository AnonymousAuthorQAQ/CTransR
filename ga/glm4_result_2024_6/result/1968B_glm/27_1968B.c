fn main() {
    let t = 3; // Number of test cases
    let test_cases = vec![
        (3, 4, String::from("abc"), String::from("abcd")),
        (5, 3, String::from("abcde"), String::from("abc")),
        (2, 4, String::from("ab"), String::from("abcd")),
    ];

    for (n, m, sn, sm) in test_cases {
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m && sn.chars().nth(i) == sm.chars().nth(j) {
            i += 1;
            j += 1;
        }
        println!("{}", i);
    }
}
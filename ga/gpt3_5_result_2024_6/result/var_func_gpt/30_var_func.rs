use std::ffi::VaList;

fn print_values(n: i32, mut vl: VaList) {
    for _ in 0..n {
        let value: i32 = vl.arg::<i32>();
        println!("Value: {}", value);
    }
}

fn main() {
    let values = vec![10, 20, 30];
    
    let vl = std::ffi::VaList::from(values);
    print_values(3, vl);
}
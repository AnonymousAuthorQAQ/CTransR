use std::ffi::VaList;

fn print_values(n: i32, vl: VaList) {
    for i in 0..n {
        let value: i32 = vl.arg();
        println!("Value: {}", value);
    }
}

fn main() {
    let vl = VaList::from(&[10, 20, 30]);
    print_values(3, vl);
}
use std::ffi::VaList;

fn print_values(n: usize, ap: &mut VaList) {
    for _ in 0..n {
        let value: i32 = ap.arg(); // Extract next argument from va_list
        println!("Value: {}", value);
    }
}

fn main() {
    let mut vl = std::ffi::VaList::start1(3, 10, 20, 30); // Initialize va_list with 3 integer arguments
    print_values(3, &mut vl);
}
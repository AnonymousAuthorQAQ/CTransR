use std::ffi::c_void;
use std::os::raw::c_int;

fn print_values(n: c_int, ...) {
    // Create a va_list to handle the variable arguments
    let args = std::ffi::VaList::new(std::ffi::VaList::new_n(args));

    // Loop through the number of items and print each one
    for i in 0..n {
        let value: c_int = args.arg();
        println!("Value: {}", value);
    }
}

fn main() {
    // Call the print_values function with the specified number of integers
    print_values(3, 10, 20, 30);
}
fn print_values(values: &[i32]) {
    for &value in values {
        println!("Value: {}", value);
    }
}

fn main() {
    // Call the print_values function with an array of integers
    print_values(&[10, 20, 30]);
}
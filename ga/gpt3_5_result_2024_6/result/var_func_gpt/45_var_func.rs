use std::ffi::VaList;

fn print_values(n: i32, vl: &mut VaList) {
    for i in 0..n {
        let value: i32 = vl.arg::<i32>().unwrap_or_default();
        println!("Value: {}", value);
    }
}

fn main() {
    // Simulate va_list behavior with an empty mutable reference
    let mut vl = VaList::new();
  
    // Call print_values function passing the total number of values followed by the variable arguments
    print_values(3, &mut vl.setup_on(10, 20, 30));
}
fn print_values(n: i32, args: ...) {
    let mut args = std::va_list::from_slice(std::slice::from_raw_parts(args, n as usize));
    for _ in 0..n {
        let value = unsafe { std::va_arg(args, i32) };
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(3, &[10, 20, 30]);
}
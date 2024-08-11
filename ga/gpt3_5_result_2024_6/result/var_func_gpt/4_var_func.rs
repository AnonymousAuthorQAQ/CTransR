#include <iostream>
#include <cstdarg>

fn print_values(n: i32, ...) {
    let mut vl: va_list;
    va_start(vl, n);
    for i in 0..n {
        let value = va_arg(vl, i32);
        println!("Value: {}", value);
    }
    va_end(vl);
}

fn main() {
    print_values(3, 10, 20, 30);
    0
}
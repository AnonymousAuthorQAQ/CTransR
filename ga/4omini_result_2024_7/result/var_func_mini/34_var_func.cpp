fn print_values(n: usize, values: &[i32]) {
    if n != values.len() {
        panic!("The number of provided values does not match the specified count.");
    }
    
    for &value in values.iter() {
        println!("Value: {}", value);
    }
}

fn main() {
    print_values(3, &[10, 20, 30]);
}
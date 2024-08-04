use std::ffi::c_void;

// Define a macro to mimic variable arguments in Rust
macro_rules! print_values {
    ($n:expr $(, $value:expr)*) => {{
        let values: &[i32] = &[$($value),*]; // Store variable arguments in a slice
        for &value in values {
            println!("Value: {}", value);
        }
    }};
}

fn main() {
    print_values!(3, 10, 20, 30);
}
use std::ffi::VaList;
use std::ffi::VaListImpl;

fn print_values(n: i32, vl: &mut VaListImpl) {
    for _ in 0..n {
        let value: i32 = vl.arg::<i32>().unwrap();
        println!("Value: {}", value);
    }
}

fn main() {
    let mut vl = VaList::from(10);
    vl.add::<i32>(20);
    vl.add::<i32>(30);

    print_values(3, &mut vl);
}
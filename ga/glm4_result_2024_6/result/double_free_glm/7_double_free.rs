use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

fn main() {
    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        ptr.write(10);
        dealloc(ptr as *mut u8, layout);
        
        // The following line is intentionally commented out as it would result in a double-free error
        // dealloc(ptr as *mut u8, layout);

        // The following line is also commented out as it would result in a use-after-free error
        // println!("{}", ptr.read());
    }
}
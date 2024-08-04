use std::ptr;
use std::mem;

fn main() {
    let ptr = Box::into_raw(Box::new(10));
    unsafe {
        // Freeing memory twice, similar to free(ptr); free(ptr); in C
        Box::from_raw(ptr);
        // Attempting to access freed memory
        // Uncomment the next line to observe a segmentation fault
        // println!("{}", *ptr);
    }
    
    // The following line will not work due to the memory already being freed
    // println!("{}", *ptr);
}
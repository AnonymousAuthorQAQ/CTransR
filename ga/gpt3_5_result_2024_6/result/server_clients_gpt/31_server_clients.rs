use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use memmap::parse_flags;
use memmap::MmapMut;

fn main() {
    // Create and open a file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o664)
        .open("temp")
        .unwrap();

    // Set the size of the file
    file.set_len(4096).unwrap();

    // Map the file into memory
    let mmap = unsafe { MmapMut::map_mut(&file).unwrap() };

    // Main loop
    let mut stop = 0;
    while stop < 10 {
        // Write to a specific offset within the memory-mapped area
        let mut p = mmap.get_mut(1024..1024 + 12).unwrap();
        p.write_all(b"Test message").unwrap();
        println!("Message written to the shared memory: {}", std::str::from_utf8(&p).unwrap());

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep to simulate delay and observe the output

        // Read from the same offset
        let read_data = &mmap[1024..1024 + 12];
        println!("Read from shared memory: {}", std::str::from_utf8(&read_data).unwrap());

        std::thread::sleep(std::time::Duration::from_secs(1)); // Sleep again to simulate delay
        stop += 1;
    }

    // memmap automatically unmaps the memory when dropped
    // Close and remove file
    std::fs::remove_file("temp").unwrap();
}
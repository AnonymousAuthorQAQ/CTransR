#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/mman.h>
#include <fcntl.h>

int main(int argc, const char *argv[]) {
    // Create and open a file
    int fd = open("temp", O_RDWR | O_CREAT, 0664);
    if (fd == -1) {
        perror("open");
        exit(1);
    }

    // Set the size of the file
    if (ftruncate(fd, 4096) == -1) {
        perror("ftruncate");
        exit(1);
    }

    // Map the file into memory
    void *ptr = mmap(NULL, 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (ptr == MAP_FAILED) {
        perror("mmap");
        exit(1);
    }

    // Main loop
    int stop = 0;
    while (stop < 10) {
        // Write to a specific offset within the memory-mapped area
        char *p = (char *)ptr + 1024;
        strcpy(p, "Test message");
        printf("Message written to the shared memory: %s\n", p);
        sleep(1); // Sleep to simulate delay and observe the output

        // Read from the same offset
        printf("Read from shared memory: %s\n", (char *)ptr + 1024);
        sleep(1); // Sleep again to simulate delay
        stop ++;
    }

    // Unmap and close everything
    if (munmap(ptr, 4096) == -1) {
        perror("munmap");
        exit(1);
    }
    

    close(fd);

    if (unlink("temp") == -1) {
        perror("unlink");
        exit(1);
    }

    return 0;
}

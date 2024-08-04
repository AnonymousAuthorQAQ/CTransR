#include <iostream>
#include <thread>
#include <atomic>

std::atomic<int> account_balance(1000000);

void withdraw() {
    int amount = 500000;  
    for (int i = 0; i < amount; i++) {
        account_balance--;
    }
}

int main() {
    std::thread t1(withdraw);
    std::thread t2(withdraw);

    t1.join();
    t2.join();

    std::cout << "Final account balance: " << account_balance << std::endl;
    return 0;
}


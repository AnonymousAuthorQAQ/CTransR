#include <iostream>

#define CPPMAX(x, y) ((x) + (y) < 5 ? (y) : (x))

int main() {
    int a, b;
    std::cin >> a >> b;

    int result = CPPMAX(a, b);
    std::cout << "Result: " << result << std::endl;

    return 0;
}



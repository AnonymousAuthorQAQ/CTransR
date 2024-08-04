#include <iostream>
#include <memory>
struct MyStruct {
    int value;
    MyStruct(int v) : value(v) {
        std::cout << "Constructor\n";
    }

    ~MyStruct() {
        std::cout << "Destructor\n";
    }
};
int main() {
    std::shared_ptr<MyStruct> ptr = std::make_shared<MyStruct>(10);
    std::weak_ptr<MyStruct> weak_ptr = ptr; // Create weak pointer
    if (auto shared_ptr = weak_ptr.lock()) { // attempt to obtain shared_ptr
    shared_ptr->value = 20; // modify data
    std::cout << "Value: " << shared_ptr->value << "\n";
    }
    return 0;
}


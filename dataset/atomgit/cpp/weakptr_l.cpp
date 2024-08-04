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

void observe(std::weak_ptr<MyStruct> weak_ptr) {
    if (auto shared_ptr = weak_ptr.lock()) {
        std::cout << "Object is alive, value: " << shared_ptr->value << "\n";
    } else {
        std::cout << "Object has been destroyed\n";
    }
}

int main() {
    std::shared_ptr<MyStruct> ptr1 = std::make_shared<MyStruct>(10);
    std::weak_ptr<MyStruct> weak_ptr1 = ptr1;

    {
        std::shared_ptr<MyStruct> ptr2 = ptr1;
        std::cout << "Inside inner scope\n";
        observe(weak_ptr1); 


        std::weak_ptr<MyStruct> weak_ptr2 = ptr2;
        if (auto shared_ptr = weak_ptr2.lock()) {
            shared_ptr->value = 20; 
            std::cout << "Value: " << shared_ptr->value << "\n";
        }
    } 

    std::cout << "Outside inner scope\n";
    observe(weak_ptr1); 


    ptr1.reset();
    observe(weak_ptr1); 

    return 0;
}

#include <iostream>
#include <cmath>

class AbstractK {
public:
    double r;
    double p;
    AbstractK(const double &r, const double &p) : r(r), p(p) {};
    virtual const bool isInside() const = 0;
    virtual const bool overlap() const = 0;
};

class K2D : public AbstractK {
public:
    K2D(const double &r, const double &p) : AbstractK(r, p) {};
    virtual const bool overlap() const override {
        return (r > p);
    }
    virtual const bool isInside() const override {
        return false;  // To be defined in more specific classes like SquareK
    }
};

class K3D : public AbstractK {
public:
    K3D(const double &r, const double &p) : AbstractK(r, p) {};
    virtual const bool overlap() const override {
        return (r > p);
    }
    virtual const bool isInside() const override {
        return false;  // To be defined in more specific classes like CubeK
    }
};

class SquareK : public K2D {
public:
    SquareK(const double &r, const double &p) : K2D(r, p) {};
    virtual const bool isInside() const override {
        return (r * r <= p);
    }
};

class CubeK : public K3D {
public:
    CubeK(const double &r, const double &p) : K3D(r, p) {};
    virtual const bool isInside() const override {
        return (r * r * r <= p);
    }
};

int main() {
    double r, p;
    std::cout << "Enter radius r and parameter p for SquareK: "<< std::endl;
    std::cin >> r >> p;
    SquareK square(r, p);
    std::cout << "SquareK isInside: " << (square.isInside() ? "Yes" : "No") << std::endl;
    std::cout << "SquareK overlap: " << (square.overlap() ? "Yes" : "No") << std::endl;

    std::cout << "Enter radius r and parameter p for CubeK: "<< std::endl;
    std::cin >> r >> p;
    CubeK cube(r, p);
    std::cout << "CubeK isInside: " << (cube.isInside() ? "Yes" : "No") << std::endl;
    std::cout << "CubeK overlap: " << (cube.overlap() ? "Yes" : "No") << std::endl;

    return 0;
}

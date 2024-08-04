use std::io;

trait Shape {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct AbstractK {
    r: f64,
    p: f64,
}

impl AbstractK {
    fn new(r: f64, p: f64) -> Self {
        AbstractK { r, p }
    }
}

struct K2D {
    base: AbstractK,
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { base: AbstractK::new(r, p) }
    }
}

impl Shape for K2D {
    fn overlap(&self) -> bool {
        self.base.r > self.base.p
    }

    fn is_inside(&self) -> bool {
        false // Should be defined in the derived struct
    }
}

struct K3D {
    base: AbstractK,
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { base: AbstractK::new(r, p) }
    }
}

impl Shape for K3D {
    fn overlap(&self) -> bool {
        self.base.r > self.base.p
    }

    fn is_inside(&self) -> bool {
        false // Should be defined in the derived struct
    }
}

struct SquareK {
    base: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { base: K2D::new(r, p) }
    }
}

impl Shape for SquareK {
    fn is_inside(&self) -> bool {
        self.base.base.r * self.base.base.r <= self.base.base.p
    }
}

struct CubeK {
    base: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { base: K3D::new(r, p) }
    }
}

impl Shape for CubeK {
    fn is_inside(&self) -> bool {
        self.base.base.r * self.base.base.r * self.base.base.r <= self.base.base.p
    }
}

fn main() {
    let mut input = String::new();

    // Input for SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let r: f64 = values[0].parse().expect("Please enter a number");
    let p: f64 = values[1].parse().expect("Please enter a number");
    
    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.base.overlap() { "Yes" } else { "No" });

    input.clear(); // Clear input for the next use

    // Input for CubeK
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<&str> = input.trim().split_whitespace().collect();
    let r: f64 = values[0].parse().expect("Please enter a number");
    let p: f64 = values[1].parse().expect("Please enter a number");
    
    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.base.overlap() { "Yes" } else { "No" });
}
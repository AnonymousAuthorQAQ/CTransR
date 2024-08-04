use std::io;
use std::cmp::Ordering;

// Define a trait for geometric shapes
trait Shape {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

// Define a struct for 2D shapes
struct K2D {
    radius: f64,
    parameter: f64,
}

impl K2D {
    fn new(radius: f64, parameter: f64) -> Self {
        K2D { radius, parameter }
    }
}

impl Shape for K2D {
    fn is_inside(&self) -> bool {
        false // To be implemented in more specific classes like SquareK
    }

    fn overlap(&self) -> bool {
        self.radius > self.parameter
    }
}

// Define a struct for 3D shapes
struct K3D {
    radius: f64,
    parameter: f64,
}

impl K3D {
    fn new(radius: f64, parameter: f64) -> Self {
        K3D { radius, parameter }
    }
}

impl Shape for K3D {
    fn is_inside(&self) -> bool {
        false // To be implemented in more specific classes like CubeK
    }

    fn overlap(&self) -> bool {
        self.radius > self.parameter
    }
}

// Define a struct for SquareK shape
struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(radius: f64, parameter: f64) -> Self {
        SquareK { k2d: K2D::new(radius, parameter) }
    }
}

impl Shape for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.radius.powi(2) <= self.k2d.parameter
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

// Define a struct for CubeK shape
struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(radius: f64, parameter: f64) -> Self {
        CubeK { k3d: K3D::new(radius, parameter) }
    }
}

impl Shape for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.radius.powi(3) <= self.k3d.parameter
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut input = String::new();

    // Prompt user for input for SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Not a number")).collect();
    let r = parts[0];
    let p = parts[1];
    let squarek = SquareK::new(r, p);
    println!("SquareK isInside: {}", squarek.is_inside());
    println!("SquareK overlap: {}", squarek.overlap());

    // Prompt user for input for CubeK
    println!("Enter radius r and parameter p for CubeK:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Not a number")).collect();
    let r = parts[0];
    let p = parts[1];
    let cubek = CubeK::new(r, p);
    println!("CubeK isInside: {}", cubek.is_inside());
    println!("CubeK overlap: {}", cubek.overlap());
}
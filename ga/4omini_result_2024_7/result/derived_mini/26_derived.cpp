use std::io;

trait AbstractK {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn is_inside(&self) -> bool {
        false // Default; should be implemented in derived structs
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn is_inside(&self) -> bool {
        false // Default; should be implemented in derived structs
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { r, p }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }
    
    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { r, p }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

fn main() {
    let mut input = String::new();
    
    // SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let square = SquareK::new(values[0], values[1]);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    // Reset input for CubeK
    input.clear();
    
    // CubeK
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let cube = CubeK::new(values[0], values[1]);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}
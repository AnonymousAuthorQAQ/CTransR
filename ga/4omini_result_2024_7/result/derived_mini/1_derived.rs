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
        false // To be defined in more specific structs like SquareK
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
        false // To be defined in more specific structs like CubeK
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
        Self { r, p }
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
        Self { r, p }
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
    
    // SquareK input
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
    let (r, p) = (parts[0], parts[1]);
    
    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear(); // Clear input buffer for next input
    
    // CubeK input
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
    let (r, p) = (parts[0], parts[1]);
    
    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}
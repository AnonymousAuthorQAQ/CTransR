use std::io;

trait AbstractK {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    r: f64,
    p: f64,
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }
}

impl AbstractK for K2D {
    fn overlap(&self) -> bool {
        self.r > self.p
    }
    
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like SquareK
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }
}

impl AbstractK for K3D {
    fn overlap(&self) -> bool {
        self.r > self.p
    }
    
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like CubeK
    }
}

struct SquareK {
    k: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { k: K2D::new(r, p) }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k.r * self.k.r <= self.k.p
    }
    
    fn overlap(&self) -> bool {
        self.k.overlap()
    }
}

struct CubeK {
    k: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { k: K3D::new(r, p) }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k.r * self.k.r * self.k.r <= self.k.p
    }
    
    fn overlap(&self) -> bool {
        self.k.overlap()
    }
}

fn main() {
    let mut input = String::new();

    // User input for SquareK
    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut params: Vec<&str> = input.trim().split_whitespace().collect();
    let r: f64 = params[0].parse().unwrap();
    let p: f64 = params[1].parse().unwrap();

    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear(); // Clear the input buffer

    // User input for CubeK
    println!("Enter radius r and parameter p for CubeK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    params = input.trim().split_whitespace().collect();
    let r: f64 = params[0].parse().unwrap();
    let p: f64 = params[1].parse().unwrap();

    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}
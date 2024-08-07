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
        false // This should be defined in derived classes
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
        false // This should be defined in derived classes
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK {
            k2d: K2D { r, p },
        }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.r * self.k2d.r <= self.k2d.p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK {
            k3d: K3D { r, p },
        }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.r * self.k3d.r * self.k3d.r <= self.k3d.p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut input = String::new();
    
    // SquareK input
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let r: f64 = parts[0].parse().expect("Invalid input for radius");
    let p: f64 = parts[1].parse().expect("Invalid input for parameter");

    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear(); // Clear the input buffer for the next input

    // CubeK input
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let r: f64 = parts[0].parse().expect("Invalid input for radius");
    let p: f64 = parts[1].parse().expect("Invalid input for parameter");

    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}
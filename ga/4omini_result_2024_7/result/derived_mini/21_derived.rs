use std::io;

trait AbstractK {
    fn r(&self) -> f64;
    fn p(&self) -> f64;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    radius: f64,
    parameter: f64,
}

impl AbstractK for K2D {
    fn r(&self) -> f64 {
        self.radius
    }

    fn p(&self) -> f64 {
        self.parameter
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like SquareK
    }

    fn overlap(&self) -> bool {
        self.radius > self.parameter
    }
}

struct K3D {
    radius: f64,
    parameter: f64,
}

impl AbstractK for K3D {
    fn r(&self) -> f64 {
        self.radius
    }

    fn p(&self) -> f64 {
        self.parameter
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like CubeK
    }

    fn overlap(&self) -> bool {
        self.radius > self.parameter
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(radius: f64, parameter: f64) -> SquareK {
        SquareK {
            k2d: K2D { radius, parameter },
        }
    }
}

impl AbstractK for SquareK {
    fn r(&self) -> f64 {
        self.k2d.r()
    }

    fn p(&self) -> f64 {
        self.k2d.p()
    }

    fn is_inside(&self) -> bool {
        self.r() * self.r() <= self.p()
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(radius: f64, parameter: f64) -> CubeK {
        CubeK {
            k3d: K3D { radius, parameter },
        }
    }
}

impl AbstractK for CubeK {
    fn r(&self) -> f64 {
        self.k3d.r()
    }

    fn p(&self) -> f64 {
        self.k3d.p()
    }

    fn is_inside(&self) -> bool {
        self.r() * self.r() * self.r() <= self.p()
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut input = String::new();

    // SquareK instance
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Parse error")).collect();
    let square = SquareK::new(values[0], values[1]);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear(); // Clear the input buffer

    // CubeK instance
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Parse error")).collect();
    let cube = CubeK::new(values[0], values[1]);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}
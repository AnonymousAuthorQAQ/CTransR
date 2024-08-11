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
        false  // Define in specific classes
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
        false  // Define in specific classes
    }

    fn overlap(&self) -> bool {
        self.radius > self.parameter
    }
}

struct SquareK {
    base: K2D,
}

impl SquareK {
    fn new(radius: f64, parameter: f64) -> Self {
        SquareK {
            base: K2D {
                radius,
                parameter,
            },
        }
    }
}

impl AbstractK for SquareK {
    fn r(&self) -> f64 {
        self.base.r()
    }
    
    fn p(&self) -> f64 {
        self.base.p()
    }
    
    fn is_inside(&self) -> bool {
        self.r() * self.r() <= self.p()
    }
    
    fn overlap(&self) -> bool {
        self.base.overlap()
    }
}

struct CubeK {
    base: K3D,
}

impl CubeK {
    fn new(radius: f64, parameter: f64) -> Self {
        CubeK {
            base: K3D {
                radius,
                parameter,
            },
        }
    }
}

impl AbstractK for CubeK {
    fn r(&self) -> f64 {
        self.base.r()
    }
    
    fn p(&self) -> f64 {
        self.base.p()
    }
    
    fn is_inside(&self) -> bool {
        self.r() * self.r() * self.r() <= self.p()
    }
    
    fn overlap(&self) -> bool {
        self.base.overlap()
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut values: Vec<f64> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let square = SquareK::new(values[0], values[1]);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });
    
    input.clear();
    
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let cube = CubeK::new(values[0], values[1]);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}
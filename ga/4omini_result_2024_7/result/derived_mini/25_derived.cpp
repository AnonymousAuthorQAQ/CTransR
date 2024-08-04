use std::io;

trait AbstractK {
    fn r(&self) -> f64;
    fn p(&self) -> f64;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn r(&self) -> f64 {
        self.r
    }
    
    fn p(&self) -> f64 {
        self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn is_inside(&self) -> bool {
        false // To be implemented in derived struct (SquareK)
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn is_inside(&self) -> bool {
        false // To be implemented in derived struct (CubeK)
    }
}

struct SquareK {
    base: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK {
            base: K2D { r, p },
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
        self.base.r * self.base.r <= self.base.p
    }
    
    fn overlap(&self) -> bool {
        self.base.overlap()
    }
}

struct CubeK {
    base: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK {
            base: K3D { r, p },
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
        self.base.r * self.base.r * self.base.r <= self.base.p
    }

    fn overlap(&self) -> bool {
        self.base.overlap()
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter radius r and parameter p for SquareK:");

    io::stdin().read_line(&mut input).unwrap();
    let mut inputs = input.trim().split_whitespace();
    let r: f64 = inputs.next().unwrap().parse().unwrap();
    let p: f64 = inputs.next().unwrap().parse().unwrap();
    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear();
    println!("Enter radius r and parameter p for CubeK:");

    io::stdin().read_line(&mut input).unwrap();
    let mut inputs = input.trim().split_whitespace();
    let r: f64 = inputs.next().unwrap().parse().unwrap();
    let p: f64 = inputs.next().unwrap().parse().unwrap();
    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}
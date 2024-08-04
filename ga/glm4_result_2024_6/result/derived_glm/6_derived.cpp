fn main() {
    let r: f64;
    let p: f64;

    println!("Enter radius r and parameter p for SquareK: ");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    r = 1.0; // Placeholder for user input
    p = 1.0; // Placeholder for user input

    let square = SquareK { r, p };
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    println!("Enter radius r and parameter p for CubeK: ");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    r = 1.0; // Placeholder for user input
    p = 1.0; // Placeholder for user input

    let cube = CubeK { r, p };
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}

trait AbstractK {
    fn r(&self) -> f64;
    fn p(&self) -> f64;

    fn is_inside(&self) -> bool {
        false
    }

    fn overlap(&self) -> bool {
        self.r() > self.p()
    }
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
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl AbstractK for CubeK {
    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }
}
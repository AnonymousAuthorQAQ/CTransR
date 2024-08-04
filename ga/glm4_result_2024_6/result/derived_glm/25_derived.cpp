trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
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
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like SquareK
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
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like CubeK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    inner: K2D,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { inner: K2D::new(r, p) }
    }

    fn r(&self) -> f64 {
        self.inner.r()
    }

    fn p(&self) -> f64 {
        self.inner.p()
    }

    fn is_inside(&self) -> bool {
        self.r().powi(2) <= self.p()
    }

    fn overlap(&self) -> bool {
        self.inner.overlap()
    }
}

struct CubeK {
    inner: K3D,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { inner: K3D::new(r, p) }
    }

    fn r(&self) -> f64 {
        self.inner.r()
    }

    fn p(&self) -> f64 {
        self.inner.p()
    }

    fn is_inside(&self) -> bool {
        self.r().powi(3) <= self.p()
    }

    fn overlap(&self) -> bool {
        self.inner.overlap()
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter radius r and parameter p for SquareK:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let (r, p) = parse_input(input);
    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    input.clear();
    println!("Enter radius r and parameter p for CubeK:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let (r, p) = parse_input(input);
    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}

fn parse_input(input: String) -> (f64, f64) {
    let parts: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    (parts[0], parts[1])
}
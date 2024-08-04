trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
    fn r(&self) -> f64;
    fn p(&self) -> f64;
}

#[derive(Debug)]
struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }

    fn is_inside(&self) -> bool {
        // To be defined in more specific structs like SquareK
        unimplemented!()
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

#[derive(Debug)]
struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }

    fn is_inside(&self) -> bool {
        // To be defined in more specific structs like CubeK
        unimplemented!()
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

#[derive(Debug)]
struct SquareK {
    k2d: K2D,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK {
            k2d: K2D::new(r, p),
        }
    }

    fn is_inside(&self) -> bool {
        self.k2d.r.powi(2) <= self.k2d.p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }

    fn r(&self) -> f64 {
        self.k2d.r
    }

    fn p(&self) -> f64 {
        self.k2d.p
    }
}

#[derive(Debug)]
struct CubeK {
    k3d: K3D,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK {
            k3d: K3D::new(r, p),
        }
    }

    fn is_inside(&self) -> bool {
        self.k3d.r.powi(3) <= self.k3d.p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }

    fn r(&self) -> f64 {
        self.k3d.r
    }

    fn p(&self) -> f64 {
        self.k3d.p
    }
}

fn main() {
    println!("Enter radius r and parameter p for SquareK:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let (r, p) = parse_input(input.trim());

    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    println!("Enter radius r and parameter p for CubeK:");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let (r, p) = parse_input(input.trim());

    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}

fn parse_input(input: &str) -> (f64, f64) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Invalid input");
    }
    let r: f64 = parts[0].parse().expect("Invalid radius");
    let p: f64 = parts[1].parse().expect("Invalid parameter");
    (r, p)
}
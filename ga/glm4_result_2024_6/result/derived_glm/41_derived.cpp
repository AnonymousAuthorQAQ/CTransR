use std::io;

// Define an abstract struct with a generic parameter 'T' to represent the dimension
struct AbstractK<T> {
    r: T,
    p: T,
}

// Implement the 'AbstractK' trait for the 'AbstractK' struct
impl<T> AbstractK<T> {
    fn new(r: T, p: T) -> Self {
        AbstractK { r, p }
    }

    // Define a default implementation for 'overlap' that returns true if 'r' is greater than 'p'
    fn overlap(&self) -> bool {
        self.r > self.p
    }

    // Define a default implementation for 'is_inside' that returns false
    fn is_inside(&self) -> bool {
        false
    }
}

// Define a 'K2D' struct for 2D shapes
struct K2D<T> {
    base: AbstractK<T>,
}

// Implement the 'K2D' struct
impl<T> K2D<T> {
    fn new(r: T, p: T) -> Self {
        K2D {
            base: AbstractK::new(r, p),
        }
    }
}

// Define a 'K3D' struct for 3D shapes
struct K3D<T> {
    base: AbstractK<T>,
}

// Implement the 'K3D' struct
impl<T> K3D<T> {
    fn new(r: T, p: T) -> Self {
        K3D {
            base: AbstractK::new(r, p),
        }
    }
}

// Define a 'SquareK' struct for squares
struct SquareK<T> {
    base: K2D<T>,
}

// Implement the 'SquareK' struct
impl<T> SquareK<T> {
    fn new(r: T, p: T) -> Self {
        SquareK {
            base: K2D::new(r, p),
        }
    }

    // Override the 'is_inside' method for squares
    fn is_inside(&self) -> bool {
        self.base.r * self.base.r <= self.base.p
    }
}

// Define a 'CubeK' struct for cubes
struct CubeK<T> {
    base: K3D<T>,
}

// Implement the 'CubeK' struct
impl<T> CubeK<T> {
    fn new(r: T, p: T) -> Self {
        CubeK {
            base: K3D::new(r, p),
        }
    }

    // Override the 'is_inside' method for cubes
    fn is_inside(&self) -> bool {
        self.base.r * self.base.r * self.base.r <= self.base.p
    }
}

fn main() {
    let mut input = String::new();

    // Input for SquareK
    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let r_square = values[0];
    let p_square = values[1];
    let square = SquareK::new(r_square, p_square);
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.base.overlap());

    // Input for CubeK
    println!("Enter radius r and parameter p for CubeK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let r_cube = values[0];
    let p_cube = values[1];
    let cube = CubeK::new(r_cube, p_cube);
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.base.overlap());
}
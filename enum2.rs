#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64), // base, height
}

impl Shape {
    // Returns the area of the shape.
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

// Returns a reference to the shape with the largest area.
fn find_largest_shape(shapes: &[Shape]) -> Option<&Shape> {
    shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap_or(std::cmp::Ordering::Equal))
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0, 6.0)];

    let total_area: f64 = shapes.iter().map(|shape| shape.area()).sum();

    println!("Total area: {:.2} sq. units", total_area);

    // Find and print the largest shape and its area.
    if let Some(largest) = find_largest_shape(&shapes) {
        println!("The largest shape is {:?} with an area of {:.2} sq. units.", largest, largest.area());
    } else {
        println!("The shapes vector is empty.");
    }
}

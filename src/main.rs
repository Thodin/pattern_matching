use geomtery::{shape_operations::calc_area, shapes::Shape};

pub mod geomtery;

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle(0.7),
        Shape::Square(0.9),
        Shape::Rectangle(1.2, 0.4),
    ];

    for shape in &shapes {
        calc_area(shape);
    }
}

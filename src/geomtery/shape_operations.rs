use std::f64::consts::PI;

use super::shapes::Shape;

pub fn calc_area(shape: &Shape) {
    match shape {
        Shape::Circle(radius) => println!(
            "Area of circle with radius {:.4}: {:.4}",
            radius,
            radius.powi(2) * PI
        ),
        Shape::Square(side) => {
            println!("Area of square with side {:.4}: {:.4}", side, side.powi(2))
        }
        Shape::Rectangle(width, height) => println!(
            "Area of rectangle with width {:.4} and height {:.4}: {:.4}",
            width,
            height,
            width * height
        ),
    }
}

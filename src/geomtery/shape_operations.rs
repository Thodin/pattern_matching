use std::f64::consts::PI;

use super::shapes::Shape;

pub fn calc_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => radius.powi(2) * PI,
        Shape::Square(side) => side.powi(2),
        Shape::Rectangle(width, height) => width * height,
    }
}

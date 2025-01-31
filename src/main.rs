use std::time::Instant;

use geomtery::{shape_operations::calc_area, shapes::Shape};
use rand::{rngs::ThreadRng, Rng};

pub mod geomtery;

fn main() {
    run_and_time(create_shapes_and_calculate_area, 1_000_000);
}

fn create_shapes_and_calculate_area(rng: &mut ThreadRng) -> f64 {
    let random_num = rng.gen_range(0.5..1.5);

    let shapes: Vec<Shape> = vec![
        Shape::Circle(random_num),
        Shape::Circle(random_num),
        Shape::Circle(random_num),
        Shape::Circle(random_num),
        Shape::Circle(random_num),
        Shape::Square(random_num),
        Shape::Square(random_num),
        Shape::Square(random_num),
        Shape::Square(random_num),
        Shape::Square(random_num),
        Shape::Rectangle(random_num, random_num),
        Shape::Rectangle(random_num, random_num),
        Shape::Rectangle(random_num, random_num),
        Shape::Rectangle(random_num, random_num),
        Shape::Rectangle(random_num, random_num),
    ];

    let mut sum = 0_f64;
    for shape in &shapes {
        sum += calc_area(shape);
    }
    sum
}

fn run_and_time<T>(fun: T, num_iterations: u32)
where
    T: Fn(&mut ThreadRng) -> f64,
{
    let mut rng = rand::thread_rng();
    let mut sum = 0_f64;

    let start = Instant::now();
    for _ in 0..num_iterations {
        sum += fun(&mut rng);
    }
    let duration = start.elapsed();

    println!("Duration: {:?}", duration);
    println!("Sum: {sum}");
}

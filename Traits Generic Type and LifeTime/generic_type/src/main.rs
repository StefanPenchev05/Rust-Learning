use std::fmt::Debug;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x_position: T,
    y_position: U,
}

impl<T: Debug, U: Debug> Point<T, U> {
    fn new(x_position: T, y_position: U) -> Self {
        Self {
            x_position,
            y_position,
        }
    }
    fn print_point(&self) {
        println!("x: {:?}, y: {:?}", self.get_x(), self.get_y())
    }

    fn get_x(&self) -> &T {
        &self.x_position
    }

    fn get_y(&self) -> &U {
        &self.y_position
    }
}

impl Point<f32, f32> {
    fn distance_from_each_other(&self) -> f32 {
        (self.x_position.powi(2) + self.y_position.powi(2)).sqrt()
    }
}

fn main() {
    let vec = vec![1, 2, 3, 4, 14, 75];
    println!("{}", largest(&vec));

    let vec = vec!['c', 'g', '/', 'f'];
    println!("{}", largest(&vec));

    let point = Point::new(4, 7.3);
    point.print_point();

    let point = Point::new(4.5 as f32, 6.5 as f32);
    println!(
        "The distance from x: {} and y: {} is {}",
        point.get_x(),
        point.get_y(),
        point.distance_from_each_other()
    );
}

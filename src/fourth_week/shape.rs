pub trait Shape {
    fn area(&self) -> f64;
}

pub fn calculate_area<T: Shape>(shape: &T) -> f64 {
    shape.area()
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Square {
    pub length: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.length * self.length
    }
}

pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

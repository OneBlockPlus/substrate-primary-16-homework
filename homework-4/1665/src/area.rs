use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub struct Square {
    pub side: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        // 计算圆的面积 = π * r²
        PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // 计算三角形的面积 = (base * hight)/2
        0.5 * self.base * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        // 计算正方形的面积 = side²
        self.side * self.side
    }
}

// 打印面积
pub fn print_area<T: Shape>(shape: &T) {
    println!("The area of the shape is: {:.2}", shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_area() {
        let circle = Circle { radius: 15.0 };
        let triangle = Triangle { base: 19.5, height: 25.3 };
        let square = Square { side: 33.0 };
        print_area(&circle);
        print_area(&triangle);
        print_area(&square);
    }
}
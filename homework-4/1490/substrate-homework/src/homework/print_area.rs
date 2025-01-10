// 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。

use std::f64::consts::PI;

pub trait MeasurableArea {
    fn area(&self) -> Result<f64, String>;
}

pub struct Circle {
    pub radius: f64,
}

impl MeasurableArea for Circle {
    fn area(&self) -> Result<f64, String> {
        if self.radius < 0.0 {
            return Err("Radius cannot be negative".to_string());
        }
        Ok(PI * self.radius * self.radius)
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl MeasurableArea for Triangle {
    fn area(&self) -> Result<f64, String> {
        if self.base < 0.0 || self.height < 0.0 {
            return Err("Base and height cannot be negative".to_string());
        }
        Ok(0.5 * self.base * self.height)
    }
}

pub struct Square {
    pub side: f64,
}

impl MeasurableArea for Square {
    fn area(&self) -> Result<f64, String> {
        if self.side < 0.0 {
            return Err("Side cannot be negative".to_string());
        }
        Ok(self.side * self.side)
    }
}

pub fn print_area<T: MeasurableArea>(shape: &T) {
    match shape.area() {
        Ok(area) => println!("The area of the shape is: {}", area),
        Err(e) => println!("Error calculating area: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.area().unwrap(), PI * 25.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle {
            base: 4.0,
            height: 3.0,
        };
        assert_eq!(triangle.area().unwrap(), 6.0);

        let invalid_triangle = Triangle {
            base: -4.0,
            height: 3.0,
        };
        assert_eq!(
            invalid_triangle.area().unwrap_err(),
            "Base and height cannot be negative"
        );
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 5.0 };
        assert_eq!(square.area().unwrap(), 25.0);

        let invalid_square = Square { side: -5.0 };
        assert_eq!(
            invalid_square.area().unwrap_err(),
            "Side cannot be negative"
        );

        let zero_square = Square { side: 0.0 };
        assert_eq!(zero_square.area().unwrap(), 0.0);
    }
}

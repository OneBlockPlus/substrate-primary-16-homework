/// 定义一个area trait用于面积计算
trait Area {
    fn area(&self) -> f64;
}

/// 定义一个结构体 Circle 圆
pub struct Circle {
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// 定义一个结构体 Triangle 表示三角形
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

 impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

/// 定义一个结构体 Square 表示正方形
pub struct Square {
    pub side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// 打印面积使用泛型约束
pub fn print_area<T: Area>(shape: &T) {
    println!("The area is: {:.2}", shape.area());
}

/// 编写单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 3.0 };
        assert!((circle.area() - 28.27).abs() < 1e-2);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle {
            base: 4.0,
            height: 5.0,
        };
        assert_eq!(triangle.area(), 10.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 2.0 };
        assert_eq!(square.area(), 4.0);
    }
}
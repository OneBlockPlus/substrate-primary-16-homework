trait Area{
    fn area(&self) -> f64;
}

struct Circle{
    radius: f64,
}

impl Area for Circle{
    fn area(&self) -> f64{
        std::f64::consts::PI * self.radius.powi(2)
    }
}

struct Rectangle{
    width: f64,
    height: f64,
}

impl Area for Rectangle{
    fn area(&self) -> f64{
        self.width * self.height
    }
}

struct Triangle{
    base: f64,
    height: f64,
}

impl Area for Triangle{
    fn area(&self) -> f64{
        0.5 * self.base * self.height
    }
}

fn print_area<T: Area>(shape: &T){
    println!("面积：{}", shape.area());
}
fn main() {
    // 创建不同类型的形状实例
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 5.0 };
    let triangle = Triangle { base: 10.0, height: 5.0 };

    // 打印每个形状的面积
    print_area(&circle);
    print_area(&rectangle);
    print_area(&triangle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle{radius: 5.0};
        print_area(circle);
        
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle{width: 5.0, height: 10.0};
        print_area(circle);
        
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle{base: 5.0, height: 10.0};
        print_area(circle);
        
    }
}

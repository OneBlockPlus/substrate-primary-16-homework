// 定义一个 `Area` trait，包含一个返回面积的方法
trait Area {
    fn area(&self) -> f64;
}

// 定义一个 `Circle` 结构体，代表圆形
struct Circle {
    radius: f64,
}

// 为 `Circle` 实现 `Area` trait
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义一个 `Triangle` 结构体，代表三角形
struct Triangle {
    base: f64,
    height: f64,
}

// 为 `Triangle` 实现 `Area` trait
impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 定义一个 `Square` 结构体，代表正方形
struct Square {
    side: f64,
}

// 为 `Square` 实现 `Area` trait
impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 定义一个泛型函数 `print_area`，它可以打印任何实现了 `Area` trait 的类型的面积
fn print_area<T: Area>(shape: &T) {
    println!("The area is {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 4.0, height: 5.0 };
    let square = Square { side: 2.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
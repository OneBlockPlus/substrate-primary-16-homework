// =============================================【question1】=============================

// 定义枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义 Trait
trait Duration {
    fn duration(&self) -> u32;
}

// 为 TrafficLight 实现 Trait
impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,   // 红灯持续 60 秒
            TrafficLight::Yellow => 5, // 黄灯持续 5 秒
            TrafficLight::Green => 30, // 绿灯持续 30 秒
        }
    }
}

// =============================================【question2】=============================

// 定义一个 Trait 用于计算面积
trait Area {
    fn area(&self) -> f64;
}

// 圆形结构体
struct Circle {
    radius: f64,
}

// 为 Circle 实现 Area
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 正方形结构体
struct Square {
    side: f64,
}

// 为 Square 实现 Area
impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 三角形结构体
struct Triangle {
    base: f64,
    height: f64,
}

// 为 Triangle 实现 Area
impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

/// 打印面积使用泛型约束
fn print_area<T: Area>(shape: &T) {
    println!("The area is: {:.2}", shape.area());
}

// 求和函数
fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers
        .iter()
        .try_fold(0u32, |acc, &num| acc.checked_add(num))
}

// =============================================【question3】=============================

fn main() {
    println!("Hello, world!");
    // question 1
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;
    println!("Red light duration: {}", red_light.duration());
    println!("Yellow light duration: {}", yellow_light.duration());
    println!("Green light duration: {}", green_light.duration());

    // question 2
    let numbers = vec![1, 2, 3, 4];
    println!("sum of numbers: {:?}", sum_u32(&numbers));

    let large_numbers = vec![u32::MAX, 1];
    println!("sum of large numbers: {:?}", sum_u32(&large_numbers)); // 溢出会返回None

    // question 3
    let circle = Circle { radius: 3.0 };
    let square = Square { side: 4.0 };
    let triangle = Triangle {
        base: 5.0,
        height: 2.0,
    };
    print_area(&circle);
    print_area(&square);
    print_area(&triangle);
}

use std::f64::consts::PI;

// 定义交通信号灯的枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个 trait，返回信号灯持续时间
trait LightDuration {
    fn duration(&self) -> u32;
}

// 为 TrafficLight 实现 LightDuration trait
impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,   // 红灯持续 60 秒
            TrafficLight::Yellow => 5, // 黄灯持续 5 秒
            TrafficLight::Green => 55, // 绿灯持续 55 秒
        }
    }
}

// 定义一个函数，用于测试信号灯持续时间
fn test_traffic_lights() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red light duration: {} seconds", red.duration());
    println!("Yellow light duration: {} seconds", yellow.duration());
    println!("Green light duration: {} seconds", green.duration());
}

// 计算整数集合的和，处理溢出
fn sum_numbers(numbers: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for &num in numbers {
        match total.checked_add(num) { // 使用 checked_add 检测溢出
            Some(value) => total = value,
            None => return None,       // 如果溢出，返回 None
        }
    }
    Some(total) // 没有溢出时，返回总和
}

// 定义一个函数，用于测试求和功能
fn test_sum_numbers() {
    let nums = vec![1, 2, 3, 4, 5];
    match sum_numbers(&nums) {
        Some(result) => println!("Sum: {}", result),
        None => println!("Overflow occurred"),
    }

    // 测试溢出情况
    let large_nums = vec![u32::MAX, 1];
    match sum_numbers(&large_nums) {
        Some(result) => println!("Sum: {}", result),
        None => println!("Overflow occurred"),
    }
}

// 定义一个 Area trait，用于计算图形面积
trait Area {
    fn area(&self) -> f64;
}

// 实现一个圆形结构体，并实现 Area trait
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// 实现一个三角形结构体，并实现 Area trait
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现一个正方形结构体，并实现 Area trait
struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 泛型函数，接收一个实现了 Area trait 的类型
fn print_area<T: Area>(shape: T) {
    println!("The area is: {:.2}", shape.area());
}

// 定义一个函数，用于测试图形面积计算
fn test_area_calculation() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 4.0, height: 6.0 };
    let square = Square { side: 3.0 };

    print_area(circle);   // 打印圆的面积
    print_area(triangle); // 打印三角形的面积
    print_area(square);   // 打印正方形的面积
}

// 主函数，作为程序的入口
fn main() {
    // 第一部分: 测试信号灯
    test_traffic_lights();

    // 第二部分: 测试求和
    test_sum_numbers();

    // 第三部分: 测试图形面积
    test_area_calculation();
}

// 定义一个枚举表示交通信号灯的不同状态
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个 Trait 来为交通信号灯提供返回时间的方法
trait TrafficLightDuration {
    fn duration(&self) -> u32; // 返回该信号灯的持续时间
}

// 为 TrafficLight 实现 TrafficLightDuration Trait
impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,     // 红灯持续 60 秒
            TrafficLight::Yellow => 5,   // 黄灯持续 5 秒
            TrafficLight::Green => 45,   // 绿灯持续 45 秒
        }
    }
}

// 实现一个函数来计算 &[u32] 中所有整数的和
fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in nums {
        sum = sum.checked_add(num)?; // 使用 checked_add 方法防止溢出
    }
    Some(sum)
}

// 定义一个 trait 用于计算面积
trait Area {
    fn area(&self) -> f64; // 返回面积
}

// 定义圆形类型
struct Circle {
    radius: f64,
}

// 为 Circle 实现 Area trait
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义三角形类型
struct Triangle {
    base: f64,
    height: f64,
}

// 为 Triangle 实现 Area trait
impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 定义正方形类型
struct Square {
    side: f64,
}

// 为 Square 实现 Area trait
impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 打印图形面积的函数，接收实现了 Area trait 的类型
fn print_area<T: Area>(shape: T) {
    println!("The area is: {}", shape.area());
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());

    // 测试 sum_u32 函数
    let nums = vec![10, 20, 30, 40];
    match sum_u32(&nums) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred!"),
    }

    // 测试 print_area 函数
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 6.0, height: 4.0 };
    let square = Square { side: 3.0 };

    print_area(circle);    // 打印圆的面积
    print_area(triangle);  // 打印三角形的面积
    print_area(square);    // 打印正方形的面积
}

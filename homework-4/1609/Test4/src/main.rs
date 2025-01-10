// 任务 1: 交通信号灯枚举及其特性
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait LightDuration {
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,   // 红灯 60 秒
            TrafficLight::Yellow => 5, // 黄灯 5 秒
            TrafficLight::Green => 45, // 绿灯 45 秒
        }
    }
}

// 任务 2: 安全求和函数
fn safe_sum(numbers: &[u32]) -> Option<u32> {
    numbers
        .iter()
        .try_fold(0u32, |acc, &num| acc.checked_add(num))
}

// 任务 3: 打印几何形状面积
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Square {
    side: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn print_area<T: Area>(shape: &T) {
    println!("The area is: {:.2}", shape.area());
}

fn main() {
    // 测试任务 1
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());

    // 测试任务 2
    let numbers = vec![10, 20, 30, u32::MAX];
    match safe_sum(&numbers) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred!"),
    }

    // 测试任务 3
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle {
        base: 4.0,
        height: 5.0,
    };
    let square = Square { side: 2.5 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}

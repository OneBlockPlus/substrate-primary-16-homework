// 定义枚举值

fn main() {
    println!("交通灯模拟器:");
    main_light();

    println!("\nu32求和:");
    main_sum();

    println!("\n计算面积:");
    main_shape();
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}


// 定义一个函数，用于计算路灯持续时间
fn main_light() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    
    // 输出路灯持续时间
    println!("Red light duration: {} seconds", red.duration());
    println!("Yellow light duration: {} seconds", yellow.duration());
    println!("Green light duration: {} seconds", green.duration());
    
}

// 定义路灯持续时间
trait LightDuration {
    fn duration(&self) -> u32;
}   

// 实现路灯持续时间
impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

// u32类型整数集合求和，检测溢出
fn main_sum() {
    
    let numbers = vec![100, 200, 300, 400, 500];
    let sum = sum_with_overflow(&numbers);
    match sum {
        Some(result) => println!("Sum: {}", result),
        None => println!("Overflow occurred"),
    }
}

// 类型的整数集合求和，检测溢出
fn sum_with_overflow(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &num| acc.checked_add(num))
}

// 泛型和泛型约束,打印原形面积
// 定义一个泛型结构体 Circle
struct Circle<T> {
    radius: T, // 泛型字段 radius
}

// 为 Circle 实现方法，使用泛型约束
impl<T> Circle<T>
where
    T: Copy + Into<f64>, 
{
    // 计算圆的面积
    fn area(&self) -> f64 {
        let radius: f64 = self.radius.into(); 
        std::f64::consts::PI * radius * radius
    }

    // 获取半径
    fn radius(&self) -> T {
        self.radius
    }
}

fn main_shape() {
    // 使用 f64 类型的 radius 创建 Circle 实例
    let circle_f64 = Circle { radius: 3.0 };
    // 打印圆的半径和面积
    println!("Circle (f64) radius: {}", circle_f64.radius());
    println!("Circle (f64) area: {:.2}", circle_f64.area());
}

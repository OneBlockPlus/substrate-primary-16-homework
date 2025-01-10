#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait TrafficLightDuration {
    fn duration(&self) -> u32;
}

impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 120,
            TrafficLight::Yellow => 5,
        }
    }
}
// 2.
fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in nums {
        sum = sum.checked_add(num)?;
    }
    Some(sum)
}
// 3.
trait Shape {
    fn area(&self) -> f64;
}

// 圆形结构体
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 三角形结构体
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height 
    }
}

// 正方形结构体
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("形状为: {}", shape.area());
}

fn main() {
    let red_light = TrafficLight::Red;
    let green_light = TrafficLight::Green;
    let yellow_light = TrafficLight::Yellow;

    println!("红灯:{}", red_light.duration());
    println!("绿灯:{}", green_light.duration());
    println!("黄灯:{}", yellow_light.duration());

    // 2
    let numbers = &[10, 20, 30, 40, 50];
    match sum_u32(numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred during summing!"),
    }

    // 3
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 10.0, height: 5.0 };
    let square = Square { side: 4.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}

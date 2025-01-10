// 定义交通信号灯枚举
#[derive(Debug, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义时长特征
pub trait Duration {
    fn duration(&self) -> u32;
}

// 为交通信号灯实现时长特征
impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 45,
        }
    }
}

// 实现u32集合求和函数
pub fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in numbers {
        sum = sum.checked_add(num)?;
    }
    Some(sum)
}

// 定义面积特征
pub trait Area {
    fn area(&self) -> f64;
}

// 定义几何图形
pub struct Circle {
    pub radius: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub struct Square {
    pub side: f64,
}

// 为几何图形实现面积特征
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 打印面积的泛型函数
fn print_area<T: Area>(shape: &T) {
    println!("图形的面积是: {}", shape.area());
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试交通信号灯持续时间
    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 30);
        assert_eq!(TrafficLight::Yellow.duration(), 3);
        assert_eq!(TrafficLight::Green.duration(), 45);
    }

    // 测试u32数组求和函数
    #[test]
    fn test_sum_u32() {
        // 正常情况测试
        assert_eq!(sum_u32(&[1, 2, 3, 4, 5]), Some(15));
        
        // 空数组测试
        assert_eq!(sum_u32(&[]), Some(0));
        
        // 溢出测试
        assert_eq!(sum_u32(&[u32::MAX, 1]), None);
    }

    // 测试几何图形面积计算
    #[test]
    fn test_shapes_area() {
        let circle = Circle { radius: 2.0 };
        let triangle = Triangle { base: 3.0, height: 4.0 };
        let square = Square { side: 5.0 };

        // 测试圆形面积
        assert!((circle.area() - 12.566370614359172).abs() < 1e-10);
        
        // 测试三角形面积
        assert_eq!(triangle.area(), 6.0);
        
        // 测试正方形面积
        assert_eq!(square.area(), 25.0);
    }
}

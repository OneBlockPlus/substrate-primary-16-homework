pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait LightDuration {
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

pub fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in numbers {
        match sum.checked_add(num) {
            Some(val) => sum = val,
            None => return None,
        }
    }
    Some(sum)
}

pub trait HasArea {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

pub struct Square {
    pub side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn print_area<T: HasArea>(shape: T) {
    println!("The area is: {}", shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 60);
        assert_eq!(TrafficLight::Yellow.duration(), 5);
        assert_eq!(TrafficLight::Green.duration(), 45);
    }

    #[test]
    fn test_sum_u32_no_overflow() {
        let numbers = [1, 2, 3, 4];
        assert_eq!(sum_u32(&numbers), Some(10));
    }

    #[test]
    fn test_sum_u32_overflow() {
        let numbers = [u32::MAX, 1];
        assert_eq!(sum_u32(&numbers), None);
    }

    #[test]
    fn test_area_calculation() {
        let circle = Circle { radius: 2.0 };
        assert_eq!(circle.area(), std::f64::consts::PI * 4.0);
        let triangle = Triangle {
            base: 3.0,
            height: 4.0,
        };
        assert_eq!(triangle.area(), 6.0);
        let square = Square { side: 5.0 };
        assert_eq!(square.area(), 25.0);
    }
}

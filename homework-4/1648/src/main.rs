#![allow(dead_code)]

// homework 1 ====================
enum TrafficLight {
    Green,
    Yellow,
    Red,
}

trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Green => 50,
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
        }
    }
}

// homework 2 ====================
fn sum(element: &[u32]) -> Option<u32> {
    element.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

// homework 3 ====================
trait Area {
    fn area(&self) -> f64;
}

fn calculate_area<T: Area>(share: T) -> f64 {
    share.area()
}

// 3.1
struct Circle {
    radius: f64,
}
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 3.2
struct Rectangle {
    width: f64,
    height: f64,
}
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 3.3
struct Triangle {
    base: f64,
    height: f64,
}
impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height * 0.5
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traffic_light() {
        assert_eq!(TrafficLight::Green.duration(), 50);
        assert_eq!(TrafficLight::Red.duration(), 60);
        assert_eq!(TrafficLight::Yellow.duration(), 10);
    }

    #[test]
    fn sum_u32() {
        let values: Vec<u32> = vec![];
        assert_eq!(sum(&values), Some(0u32));

        let values = vec![1, 3, 5, 7];
        assert_eq!(sum(&values).is_some(), true);

        let values = vec![u32::MAX, 100u32];
        assert_eq!(sum(&values).is_none(), true);
    }

    #[test]
    fn shape_test() {
        // circle
        let circle = Circle { radius: 1.23 };
        assert_eq!(calculate_area(circle), std::f64::consts::PI * 1.23 * 1.23);

        // Rectangle
        let rect = Rectangle {
            width: 1.23,
            height: 1.23,
        };
        assert_eq!(calculate_area(rect), 1.23 * 1.23);

        // Triangle
        let triangle = Triangle {
            base: 1.23,
            height: 2.46,
        };
        assert_eq!(calculate_area(triangle), 1.23 * 2.46 * 0.5);
    }
}

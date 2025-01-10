use std::f64::consts::PI;

////////////////////////////////
enum TrafficLight {
    Red(u8),
    Green(u8),
    Yellow(u8),
}

trait Timer {
    fn get_time(self) -> u8;
}

impl Timer for TrafficLight {
    fn get_time(self) -> u8 {
        let res = match self {
            TrafficLight::Red(r) => r,
            TrafficLight::Green(g) => g,
            TrafficLight::Yellow(y) => y,
        };
        res
    }
}

////////////////////////////////
fn sum(array: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for e in array {
        let (s, is_overflowing) = sum.overflowing_add(*e);
        if is_overflowing {
            return None;
        }
        sum = s;
    }
    Some(sum)
}  

////////////////////////////////
trait AreaSome {
    fn get_area(self) -> f64;
}

fn print_area<T: AreaSome>(shape: T) {
    println!("area: {}", shape.get_area());
}

struct Circle(u32);
impl AreaSome for Circle {
    fn get_area(self) -> f64 {
        PI * self.0 as f64 * self.0 as f64
    }
}

struct Rectangle{ w: u32, h: u32}
impl AreaSome for Rectangle {
    fn get_area(self) -> f64 {
        self.w as f64 * self.h as f64
    }
}


fn main() {
    let r = TrafficLight::Red(5);
    assert!(r.get_time() == 5u8, "incorrect time value");
    
    assert!(sum(&[1, 2, 3]) == Some(6), "wrong sum");
    assert!(sum(&[1, 2, u32::MAX]) == None, "wrong sum");
 
    print_area(Circle(1));
    print_area(Rectangle {w: 5, h: 5});
}


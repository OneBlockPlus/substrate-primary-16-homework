

//Traffic Light Enum with Trai

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
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 25,
        }
    }
}

//Function to Sum u32 Integers
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



//Function to Print Area of Geometric Shapes
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("Area: {}", shape.area());
}
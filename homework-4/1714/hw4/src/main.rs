// Question 1
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Duration {
    fn get_duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn get_duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 30,
        }
    }
}

// ------

// Question 2

fn sums(members: &[u32]) -> Option<u32> {
    let result = 0_u32;
    for s in members {
        result.checked_add(*s)?;
    }

    Some(result)
}

// ------

// Question 3
use std::f64::consts::PI;
trait Area {
    fn get_area(&self) -> f64;
}

struct Circle {
    r: u32,
}

struct Triangle {
    height: u32,
    bot: u32,
}
struct Square {
    a: u32,
}

impl Area for Circle {
    fn get_area(&self) -> f64 {
        PI * self.r as f64 * self.r as f64
    }
}

impl Area for Triangle {
    fn get_area(&self) -> f64 {
        0.5_f64 * self.bot as f64 * self.height as f64
    }
}

impl Area for Square {
    fn get_area(&self) -> f64 {
        self.a as f64 * self.a as f64
    }
}

fn main() {
    println!("Hello, world!");
}

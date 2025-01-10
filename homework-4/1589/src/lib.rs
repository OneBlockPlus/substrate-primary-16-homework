pub enum TrafficLight {
    Green,
    Red,
    Yellow
}

pub fn get_light_time(light: TrafficLight) -> u32 {
    match light {
        TrafficLight::Green => 50,
        TrafficLight::Red => 40,
        TrafficLight::Yellow => 10
    }
}

pub fn get_list_sum(list: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0_u32;
    for &num in list {
        match sum.checked_add(num) {
            None => return None,
            Some(result) => sum = result
        };
    }

    Some(sum)
}

pub trait Shape {
    fn get_area(&self) -> f64;
}

pub struct Square {
    side: f64
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side: (side) }
    }
}

impl Shape for Square {
    fn get_area(&self) -> f64 {
        self.side * self.side
    }
}

pub struct Circle {
    radius: f64
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius: (radius) }
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

pub struct Triangle {
    base: f64,
    height: f64
}

impl Triangle {
    pub fn new(base: f64, height: f64) -> Self {
        Self { base: (base), height: (height)}
    }
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}


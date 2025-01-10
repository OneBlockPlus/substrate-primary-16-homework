pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }
}

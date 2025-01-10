pub trait TrafficLight{
    fn duration(&self)->u32;
}

#[derive(Debug)]
pub enum TrafficLightType {
    Red,
    Yellow,
    Green,
}

impl TrafficLight for TrafficLightType {
    fn duration(&self) -> u32 {
        match self {
            TrafficLightType::Red => 20,
            TrafficLightType::Yellow => 5,
            TrafficLightType::Green => 10,
        }
    }
}

trait TrafficLight {
    fn duration(&self) -> u32;
}

pub enum TrafficLightEnum {
    RedLight{ duration: u32},
    YellowLight{ duration: u32},
    BlueLight{ duration: u32},
}

impl TrafficLight for TrafficLightEnum {
    fn duration(&self) -> u32 {
        match self {

        }
    }
}
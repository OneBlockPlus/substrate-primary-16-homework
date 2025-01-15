/**
为枚举交通信号灯实现一个 trait，trait 里包含一个返回时间的方法，不同的灯持续的时间不同
*/

pub mod traffic_lights{
    pub enum TrafficLights{
    REDLIGHT,   
    GREENLIGHT, 
    YELLOWLIGHT 
}

pub trait SignalTime{
     fn duration(&self) -> u8;
    
}

impl SignalTime for TrafficLights {
    fn duration(&self)->u8{
        match &self {
            TrafficLights::REDLIGHT => 30,
            TrafficLights::GREENLIGHT=> 60,
            TrafficLights::YELLOWLIGHT=> 5
        }
    }
}
}
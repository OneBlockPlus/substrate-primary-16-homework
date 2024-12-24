// 为枚举交通信号灯实现一个 trait，trait 里包含一个返回时间的方法，不同的灯持续的时间不同；


use std::time::Duration;
//定义交通信号灯的枚举
pub enum  TrafficSignal {
    Green ,
    Yellow,
    Red,
}
 pub trait TrafficLight {
    fn duration(&self) ->Duration;
}
impl TrafficLight for  TrafficSignal {
    fn duration(&self) -> Duration {
        match self {
            TrafficSignal::Red => Duration::from_secs(20),
            TrafficSignal::Yellow => Duration::from_secs(5),
            TrafficSignal::Green => Duration::from_secs(30),
        }
    }

}


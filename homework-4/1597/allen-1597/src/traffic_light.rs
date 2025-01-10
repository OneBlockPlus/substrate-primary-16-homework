/// 定义一个枚举表示交通信号灯
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// 定义一个 Trait 用于返回交通信号灯持续时间  u16范围0到65535
pub trait LightDuration {
    fn duration(&self) -> u16;
}

/// 为 TrafficLight 实现 LightDuration Trait
impl LightDuration for TrafficLight {
    fn duration(&self) -> u16 {
        match self {
            // 红灯120s
            TrafficLight::Red => {120}
            // 黄灯3s
            TrafficLight::Yellow => {3}
            // 绿灯60s
            TrafficLight::Green => {60}
        }
    }
}

/// 编写单元测试
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_light_duration() {
        let red = TrafficLight::Red;
        let yellow = TrafficLight::Yellow;
        let green = TrafficLight::Green;

        assert_eq!(red.duration(), 120);
        assert_eq!(yellow.duration(), 3);
        assert_eq!(green.duration(), 60);
    }
}
// 为枚举交通信号灯实现一个 trait，trait 里包含一个返回时间的方法，不同的灯持续的时间不同；

// 定义一个名为 TrafficLight 的 trait，其中包含一个返回时间的方法 duration
pub trait TrafficLight {
    fn duration(&self) -> u32;
}

// 定义一个名为 TrafficLightEnum 的枚举，用于表示交通信号灯的不同状态
// 每个变体都包含一个 duration 字段，表示该状态持续的时间
#[derive(Debug)]
pub enum TrafficLightEnum {
    Red { duration: u32 },
    Yellow { duration: u32 },
    Green { duration: u32 },
}

// 为 TrafficLightEnum 实现 TrafficLight trait
impl TrafficLight for TrafficLightEnum {
    // 实现 duration 方法，根据枚举的变体返回相应的时间
    fn duration(&self) -> u32 {
        // 使用 match 语句匹配枚举的变体，并返回对应的 duration 值
        match self {
            TrafficLightEnum::Red { duration } => *duration, // 如果是 Red 变体，返回 duration 的值
            TrafficLightEnum::Yellow { duration } => *duration, // 如果是 Yellow 变体，返回 duration 的值
            TrafficLightEnum::Green { duration } => *duration, // 如果是 Green 变体，返回 duration 的值
        }
    }
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        let red_light = TrafficLightEnum::Red { duration: 30 };
        let yellow_light = TrafficLightEnum::Yellow { duration: 5 };
        let green_light = TrafficLightEnum::Green { duration: 45 };

        assert_eq!(red_light.duration(), 30); // 断言 Red 变体的 duration 值为 30
        assert_eq!(yellow_light.duration(), 5); // 断言 Yellow 变体的 duration 值为 5
        assert_eq!(green_light.duration(), 45); // 断言 Green 变体的 duration 值为 45
    }

    #[test]
    fn test_trait() {
        let red_light = TrafficLightEnum::Red { duration: 30 };
        let yellow_light = TrafficLightEnum::Yellow { duration: 5 };
        let green_light = TrafficLightEnum::Green { duration: 45 };

        assert_eq!(red_light.duration(), 30); // 断言 Red 变体的 duration 值为 30
        assert_eq!(yellow_light.duration(), 5); // 断言 Yellow 变体的 duration 值为 5
        assert_eq!(green_light.duration(), 45); // 断言 Green 变体的 duration 值为 45
    }
}

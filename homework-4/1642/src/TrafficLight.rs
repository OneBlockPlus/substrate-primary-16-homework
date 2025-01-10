// 定义一个 `TrafficLight` trait，包含一个返回时间的方法
trait TrafficLight {
    fn duration(&self) -> u32;
}

// 定义一个 `RedLight` 结构体，代表红灯
struct RedLight;

// 为 `RedLight` 实现 `TrafficLight` trait
impl TrafficLight for RedLight {
    fn duration(&self) -> u32 {
        // 红灯持续时间，例如 10 秒
        10
    }
}

// 定义一个 `YellowLight` 结构体，代表黄灯
struct YellowLight;

// 为 `YellowLight` 实现 `TrafficLight` trait
impl TrafficLight for YellowLight {
    fn duration(&self) -> u32 {
        // 黄灯持续时间，例如 5 秒
        5
    }
}

// 定义一个 `GreenLight` 结构体，代表绿灯
struct GreenLight;

// 为 `GreenLight` 实现 `TrafficLight` trait
impl TrafficLight for GreenLight {
    fn duration(&self) -> u32 {
        // 绿灯持续时间，例如 15 秒
        15
    }
}

fn main() {
    let red_light = RedLight;
    let yellow_light = YellowLight;
    let green_light = GreenLight;

    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());
}
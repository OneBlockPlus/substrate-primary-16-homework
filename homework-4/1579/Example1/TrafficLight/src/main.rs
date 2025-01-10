

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait LightDuration {
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 40,
        }
    }
}

fn main(){
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("红灯：{}",red.duration());
    println!("黄灯：{}",yellow.duration());
    println!("绿灯：{}",green.duration());
}

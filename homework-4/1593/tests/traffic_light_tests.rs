use homework4::traffic_light::{Duration, TrafficLight};

#[test]
fn test_traffic_light_duration() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    assert_eq!(red.duration(), 60);
    assert_eq!(yellow.duration(), 5);
    assert_eq!(green.duration(), 30);
}

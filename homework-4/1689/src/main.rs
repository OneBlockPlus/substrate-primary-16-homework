use demo::homework::*;

fn main() {
    let green = TrafficLight::Green(30);
    assert_eq!(green.get_duration(), 30);

    let circle = Circle { radius: 2.0 };
    assert_eq!(calc_area(&circle), 12.566370614359172);

    let collection = vec![1, 2, 3, 4, 5];
    assert_eq!(get_sum(&collection), Some(15));
}

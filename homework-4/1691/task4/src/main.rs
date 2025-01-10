mod question;

use crate::question::{ShapeTypes, Shapes};
use crate::question::TrafficLight::{TrafficLight, TrafficLightType};
use crate::question::sum::sum;

fn main() {
    // question 1
    let red_light = TrafficLightType::Red.duration();
    let yellow_light = TrafficLightType::Yellow.duration();
    let green_light = TrafficLightType::Green.duration();
    println!("question1 -> red:{:?}, yellow:{:?}, green:{:?}",red_light, yellow_light, green_light);

    // question 2
    let sum_set: &[u32] = &[1,2,3,4];
    println!("question1 -> sum: {:?}",sum(sum_set));

    // question 3
    let triangle = ShapeTypes::Triangle(10.0, 5.0).area();
    let circle = ShapeTypes::Circle(7.0).area();
    let square = ShapeTypes::Square(10.0).area();
    println!("question3 -> triangle: {:.2?}, circle: {:.2?}, square: {:.2?}",triangle, circle,square);

}
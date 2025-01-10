use homework4::TrafficLight;
use homework4::LightDuration;
use homework4::sum_u32;

mod area;
use area::{Circle, Square, Triangle, print_area};

fn main() {
    let red_traffic_light = TrafficLight::Red;
    let yellow_traffic_light = TrafficLight::Yellow;
    let green_traffic_light = TrafficLight::Green;
    println!("Red traffic light: {}", red_traffic_light.get_duration());
    println!("Yellow traffic light: {}", yellow_traffic_light.get_duration());
    println!("Green traffic light: {}", green_traffic_light.get_duration());

    let numbers = vec![1, 2, 3, 4, 5];
    // if let Some(sum) = world_hello::sum_u32(&numbers) {
    //     println!("Sum of numbers: {}", sum);
    // }
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum of numbers: {}", sum),
        None => println!("Sum of numbers is None"),
    }

    let circle = Circle { radius: 30.0 };
    let square = Square { side: 10.0 };
    let triangle = Triangle { base: 9.5, height: 15.0 };
    print_area(&circle);
    print_area(&square);
    print_area(&triangle);
}
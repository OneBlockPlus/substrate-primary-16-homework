use homework4::geometry::{print_area, Circle, Square, Triangle};
use homework4::math::sum_u32;
use homework4::traffic_light::{Duration, TrafficLight};

fn main() {
    let mut light = TrafficLight::Red;
    println!("Red light duration: {} seconds", light.duration());
    light = TrafficLight::Yellow;
    println!("Yellow light duration: {} seconds", light.duration());
    light = TrafficLight::Green;
    println!("Green light duration: {} seconds", light.duration());

    let numbers = [1, 2, 3, u32::MAX];
    match sum_u32(&numbers) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred!"),
    }

    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let square = Square { side: 2.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}

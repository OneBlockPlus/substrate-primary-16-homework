use trafficlights::{print_area, sum_u32, Circle, LightDuration, Square, TrafficLight, Triangle};

fn main() {
    // Demonstrate TrafficLight and LightDuration
    let red_light = TrafficLight::Red;
    println!("Red light duration: {} seconds", red_light.duration());

    let yellow_light = TrafficLight::Yellow;
    println!("Yellow light duration: {} seconds", yellow_light.duration());

    let green_light = TrafficLight::Green;
    println!("Green light duration: {} seconds", green_light.duration());

    // Demonstrate sum_u32
    let numbers1 = [1, 2, 3, 4, 5];
    match sum_u32(&numbers1) {
        Some(sum) => println!("Sum of numbers: {}", sum),
        None => println!("Overflow occurred!"),
    }

    let numbers2 = [u32::MAX, 1];
    match sum_u32(&numbers2) {
        Some(sum) => println!("Sum of numbers: {}", sum),
        None => println!("Overflow occurred!"),
    }

    // Demonstrate HasArea and print_area
    let circle = Circle { radius: 3.0 };
    print_area(circle);

    let triangle = Triangle {
        base: 4.0,
        height: 5.0,
    };
    print_area(triangle);

    let square = Square { side: 6.0 };
    print_area(square);
}

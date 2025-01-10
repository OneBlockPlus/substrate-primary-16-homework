use substrate_homework::{
    print_area, sum_u32, Circle, Square, TrafficLight, TrafficLightEnum, Triangle,
};

fn main() {
    let red = TrafficLightEnum::Red { duration: 30 };
    let yellow = TrafficLightEnum::Yellow { duration: 5 };
    let green = TrafficLightEnum::Green { duration: 60 };

    println!("Red duration: {}", red.duration());
    println!("Yellow duration: {}", yellow.duration());
    println!("Green duration: {}", green.duration());

    let numbers1 = &[1, 2, 3];
    let result1 = sum_u32(numbers1);
    println!("Sum of {:?} is: {:?}", numbers1, result1);

    let numbers2 = &[u32::MAX, 1];
    let result2 = sum_u32(numbers2);
    println!("Sum of {:?} is: {:?}", numbers2, result2);

    let numbers3 = &[];
    let result3 = sum_u32(numbers3);
    println!("Sum of {:?} is: {:?}", numbers3, result3);

    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 4.0,
        height: 3.0,
    };
    let square = Square { side: 2.0 };
    let invalid_circle = Circle { radius: -5.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
    print_area(&invalid_circle);
}

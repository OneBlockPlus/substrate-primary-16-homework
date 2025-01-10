mod  traffic_light;
mod cal_area;
mod math_sum;

use traffic_light::TrafficLight;
use traffic_light::LightDuration;

use cal_area::Circle;
use cal_area::Triangle;
use cal_area::Square;
use cal_area::print_area;
use math_sum::sum_u32;



fn main() {
    println!("Hello, world!");
    println!("*************homework4*************");
    println!("作业1：红黄绿灯对应时间");
    let red = TrafficLight::Red;
    println!("Red duration: {}", red.duration());

    let  yellow = TrafficLight::Yellow;
    println!("Yellow duration: {}", yellow.duration());

    let  green = TrafficLight::Green;
    println!("Green duration: {}", green.duration());
    println!("作业2：整数求和");
    let numbers1 = &[1, 2, 3];
    let result1 = sum_u32(numbers1);
    println!("求和结果: {:?}", result1);

    println!("作业2：计算面积");
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 4.0,
        height: 3.0,
    };
    let square = Square { side: 2.0 };
    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
   
}

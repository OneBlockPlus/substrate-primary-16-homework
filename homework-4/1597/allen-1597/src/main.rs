use allen_1597::{area_calculation, math_sum, traffic_light};
use allen_1597::traffic_light::LightDuration;

fn main() {
    // 1. 信号灯的持续时间
    println!("===========打印信号灯持续时间==============");
    let red = traffic_light::TrafficLight::Red;
    println!("The duration of the red light is {} seconds", red.duration());
    let yellow = traffic_light::TrafficLight::Yellow;
    println!("The duration of the yellow light is {} seconds", yellow.duration());
    let green = traffic_light::TrafficLight::Green;
    println!("The duration of the green light is {} seconds", green.duration());

    println!();
    println!("===========打印图形面积==============");
    // 2. 打印圆形、三角形和正方形的面积
    area_calculation::print_area(&area_calculation::Circle{radius: 5.0});
    area_calculation::print_area(&area_calculation::Triangle{base: 3.0, height: 4.0});
    area_calculation::print_area(&area_calculation::Square{side: 2.0});

    println!();
    println!("===========打印数据是否溢出==============");

    // 3.1 不溢出返回Some(15)，溢出返回None
    let result = math_sum::sum_with_overflow_check(&[1, 2, 3, 4, 5]);
    match result {
        Some(sum) => println!("The sum is {}", sum),
        None => println!("Overflow occurred!"),
    }

    // 3.2 数据溢出打印Overflow occurred!
    let numbers = [1, 2, 3, u32::MAX];
    match math_sum::sum_with_overflow_check(&numbers) {
        Some(sum) => println!("The sum is {}", sum),
        None => println!("Overflow occurred!"),
    }

}
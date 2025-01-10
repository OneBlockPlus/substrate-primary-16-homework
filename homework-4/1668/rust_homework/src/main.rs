
use rand::{thread_rng, Rng};
use rust_homework::array_sum_u32::array_sum_u32;
use rust_homework::geometry_area::{print_area, CircleArea, TriangleArea, SquareArea, RectangleArea};
use rust_homework::traffic_light::{TrafficLight, TrafficSignal};
fn main() {

    //1.为枚举交通信号灯实现一个 trait，trait 里包含一个返回时间的方法，不同的灯持续的时间不同；
    let mut rng = thread_rng(); // 创建随机数生成器
    let signal1 = rng.gen_range(1..3); // 生成一个0到2的随机数，对应红绿黄三种信号灯
    let signal = match signal1 {
        0 => TrafficSignal::Red,
        1 => TrafficSignal::Yellow,
        2 => TrafficSignal::Green,
        _ => unreachable!(),
    };

    println!("交通灯持续{}秒",signal.duration().as_secs());
    //2. 实现一个函数，为 u32 类型的整数集合求和，参数类型为 &[u32]，返回类型为 Option，溢出时返回 None；
    let numbers1 = &vec![5, 7, 9];
    let result1 = array_sum_u32(numbers1);
    println!("Sum of {:?} is: {:?}", numbers1, result1);

    let numbers2 = &vec![u32::MAX, 1];
    let result2 = array_sum_u32(numbers2);
    println!("Sum of {:?} is: {:?}", numbers2, result2);

    let numbers3 = &vec![];
    let result3 =array_sum_u32(numbers3);
    println!("Sum of {:?} is: {:?}", numbers3, result3);
    //3. 打印圆形、三角形和正方形,长方形的面积
    print_area(&CircleArea{radius: 5.05});
    print_area(&TriangleArea{a: 4.0, b: 5.0,c:6.0});
    print_area(&SquareArea{side_length: 2.0});
    print_area(&RectangleArea{side_length: 2.0,side_width: 4.0});
    print_area(&RectangleArea{side_length: -2.0,side_width: 4.0});
    println!();



}

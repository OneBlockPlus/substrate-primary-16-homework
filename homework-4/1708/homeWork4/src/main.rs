use crate::traffic_lights::traffic_lights:: *;
use crate::set_sum::set_sum::int_set_sum;
use crate::calculate_area::calculate_area:: *;

pub mod traffic_lights;
pub mod set_sum;
pub mod calculate_area;

fn main() {
    // 信号灯
    let time = TrafficLights::REDLIGHT.duration();
    println!("信号灯等待时间：{}", time);
    // 整数求和
    let sum = int_set_sum(&[1,2,3,4]);
    println!("整数的和为：{}", sum.unwrap());
    // 圆形
    let circle = Circle{radius:5.0,pi:3.14};
    println!("圆形面积为：{}",shapes_calculate_area(circle));
    // 三角形
    let triangle = Triangle{bottom:3.0,high:2.0};
    println!("三角形面积为：{}",shapes_calculate_area(triangle));
    // 正方形
    let square = Square{side:4.0};
    println!("正方形面积为：{}",shapes_calculate_area(square));
}

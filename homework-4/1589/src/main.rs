use multi_files::{TrafficLight, Shape};

fn main() {
    println!("Homework4 Display: ");

    // Case 1, Traffic light duration time
    let time1 = multi_files::get_light_time(TrafficLight::Green);
    let time2 = multi_files::get_light_time(TrafficLight::Red);
    let time3 = multi_files::get_light_time(TrafficLight::Yellow);

    println!("Duration: Red = {time2}, Green = {time1}, Yellow = {time3}");

    let my_list = [2, 3, 4, 5, 6];
    let list_sum = multi_files::get_list_sum(&my_list);
    match list_sum {
        Some(total) => println!("[Success]list sum = {total}"),
        None => println!("[Failed]List Sum Overflow!")
    }

    let my_list = vec![u32::MAX, 1];
    let list_sum = multi_files::get_list_sum(&my_list);
    match list_sum {
        Some(total) => println!("[Success]list sum = {total}"),
        None => println!("[Failed]List Sum Overflow!")
    }

    let circle = multi_files::Circle::new(3.5);
    let triangle = multi_files::Triangle::new(2.5, 4.5);
    let square = multi_files::Square::new(1.5);
    println!("Circle area = {}, Triangle area = {}, Square area = {}",
        circle.get_area(), triangle.get_area(), square.get_area());
}

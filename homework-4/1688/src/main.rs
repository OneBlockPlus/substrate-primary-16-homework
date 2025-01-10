fn main() {
    let green_light: TrafficLight = TrafficLight::Green;
    println!(
        "Mission One == Green Light Duration:{}",
        green_light.duration()
    );

    println!("Mission Two == Get Sum Value");
    let list = vec![1, 2, 3, 4, 5, 6];
    let overList = vec![u32::MAX, 1];

    match add(&list) {
        Some(res) => println!("add: Sum: {}", res),
        None => println!("add: u32 溢出"),
    }

    if let Some(res) = add2(&overList) {
        println!("add2: Sum = {}", res);
    } else {
        println!("add2: u32 溢出")
    }

    println!("Mission Three == Area compute");
    let circle1: Circle = Circle{radius: 1.0};
    let triangle1: Triangle = Triangle{height: 1f64, bottom: 1f64};
    let rectangle1 = Rectangle{height:1.0, width:1f64};

    // 记得加上 &取地址  否则就会把ownner转为函数！！！！
    print_area(&circle1);
    print_area( &triangle1);
    print_area(&rectangle1);


}

//===============================================为枚举交通信号灯实现一个 trait，trait 里包含一个返回时间的方法，不同的灯持续的时间不同；
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// trait 就是行为/能力 类似Interface
trait LightDuration {
    // &self就是声明传进来的是引用 不消耗owner  &mut self 运行改变值 不消耗所有权   self 转移所有权
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        // 此时&self指向TrafficLight
        match self {
            // 枚举使用::来访问  而不是.  对于struct的实例则用.调用实例属性
            TrafficLight::Green => 10,
            TrafficLight::Yellow => 20,
            TrafficLight::Red => 30,
        }
    }
}

//============================================ 实现一个函数，为 u32 类型的整数集合求和，参数类型为 &[u32]，返回类型为 Option，溢出时返回 None；
fn add(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0u32;
    for &num in numbers {
        sum = sum.checked_add(num)?;
        //使用? checked_add返回时Option  当它返回some时 ?就会解包返回里面的值  none就返回none
        // result 返回OK时解包  Err时返回Err
    }

    Some(sum)
}

fn add2(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;

    for &num in numbers {
        // if let相当于一种情况的match  当sum.checked_add返回值是Some时就会加进去  反之则else返回None
        if let Some(new_sum) = sum.checked_add(num) {
            sum = new_sum;
        } else {
            return None; // 如果溢出，直接返回 None
        }
    }

    Some(sum)
}

// ========================================实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。
use std::f64::consts::PI;

// 定义面积trait
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Triangle {
    bottom: f64,
    height: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// 为他们实现trait
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.bottom * self.height / 2f64
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

// :用来指定格式   {:.precision}  也可以对齐 填充 百分比之类的
// println!("{:>6.2}", 3.14); // 输出： "  3.14" （右对齐，宽度为6）
// println!("{:<6.2}", 3.14); // 输出： "3.14  " （左对齐，宽度为6）
// println!("{:0>6.2}", 3.14); // 输出： "003.14" （右对齐，空白用0填充）
// let ratio = 0.1234;
// println!("{:.2}%", ratio * 100.0); // 输出： "12.34%"

// &T 别忘了 否则把ownner给到函数了
fn print_area<T: Area>(shape: &T) {
    println!("Area is: {:.2}", shape.area());
}

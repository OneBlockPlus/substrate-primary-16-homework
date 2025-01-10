fn main() {
    let c = Circle{radius: 5.0};
    let t = Triangle{side_a: 3.0, side_b: 4.0, side_c: 5.0};
    let s = Square{side: 10.0};

    get_area(c);
    get_area(t);
    get_area(s);
}

trait LightTime{
    fn light_seconds(&self)->u32;
}

trait Area{
    fn name(&self)->String;
    fn calc_area(&self)->f64;
}

#[derive(Copy, Clone)]
enum TrafficLight {
    Red,
    Yellow,
    Green
}

/// 交通信号灯时间
impl LightTime for TrafficLight {
    fn light_seconds(&self)->u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 60
        }
    }
}

/// 整数集合求和
fn  accumulation(arr: &[u32])->Option<u32>{
    let mut total:u32 = 0;
    for i in arr {
        total = total.checked_add(*i)?;
    }
    Some(total)
}


struct Circle{
    radius: f64
}

struct Triangle{
    side_a: f64,
    side_b: f64,
    side_c: f64
}

struct Square{
    side: f64
}

impl Area for Circle{
    fn name(&self)->String{
        String::from("Circle")
    }

    fn calc_area(&self)->f64{
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Area for Square{

    fn name(&self)->String{
        String::from("Square")
    }
    fn calc_area(&self)->f64 {
        self.side * self.side
    }
}

impl Area for Triangle{
    fn name(&self)->String{
        String::from("Triangle")
    }

    fn calc_area(&self)->f64{
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
}

/// 打印图形面积
fn get_area<T>(shape: T) where T: Area{
    let v: f64 = shape.calc_area();
    println!(" shape {} area is: {}", shape.name(), v);
}

#[test]
fn test(){
    let light_red = TrafficLight::Red;
    let light_green = TrafficLight::Green;
    let light_yellow = TrafficLight::Yellow;

    assert_eq!(light_red.light_seconds(), 60);
    assert_eq!(light_green.light_seconds(), 60);
    assert_eq!(light_yellow.light_seconds(), 10);

    let v: Vec<u32> = (1..=100).collect();
    assert_eq!(accumulation(&v), Some(5050));

}
use std::io;

fn main() {
    let mut a: i32 = 0;
    a = 2;
    let mut guess = String::new();
    guess = "123".to_string();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("{}", a);
    println!("Hello, world!");

    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 5.0 };
    print_area(circle);
    print_area(triangle);
    print_area(square);
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Duration {
    fn duration(&self) -> u8;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 40,
            TrafficLight::Yellow => 30,
        }
    }
}

fn sum_u32(numbers: &[u32]) -> Option<u32>{
    let mut sum: u32 = 0;
    for &num in numbers {
        sum = sum.checked_add(num)?;
    }
    Some(sum)
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Square {
    side: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(shape: T) {
    println!("The area of the shape is {}", shape.area());
}

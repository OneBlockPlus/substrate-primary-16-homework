enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait LightDuration {
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 55,
        }
    }
}

fn sum_u32_slice(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &num| acc.checked_add(num))
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(shape: &T) {
    println!("The area is: {}", shape.area());
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;
    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());

    let numbers = [1, 2, 3, 4, 5];
    match sum_u32_slice(&numbers) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred"),
    }

    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 2.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}




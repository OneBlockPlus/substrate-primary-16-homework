//
trait TrafficLight {
    fn duration(&self) -> u32;
}


#[allow(dead_code)]
enum Signal {
    Red,
    Yellow,
    Green,
}

impl TrafficLight for Signal {
    fn duration(&self) -> u32 {
        match self {
            Signal::Red => 60,     // 红灯持续 60 秒
            Signal::Yellow => 5,   // 黄灯持续 5 秒
            Signal::Green => 30,   // 绿灯持续 30 秒
        }
    }
}

//
fn safe_sum(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

//
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
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn print_area<T: Area>(shape: T) {
    println!("The area is: {}", shape.area());
}

fn main() {
    // 
    let red_light = Signal::Red;
    println!("Red light duration: {} seconds", red_light.duration());

    let green_light = Signal::Green;
    println!("Green light duration: {} seconds", green_light.duration());

    // 
    let numbers = vec![1, 2, 3, 4];
    match safe_sum(&numbers) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred!"),
    }

    let large_numbers = vec![u32::MAX, 1];
    match safe_sum(&large_numbers) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred!"),
    }

    // 
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 2.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}

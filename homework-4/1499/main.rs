// course 4 homework by no.1499
// Task 1. Implement a trait for the enum of traffic lights, which contains a method that returns the 
// duration of each light. The duration of each light is different. 

// traffic Light enum
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// lightDuration trait to get the duration of different lights
trait LightDuration {
    // returns the duration in seconds
    fn duration(&self) -> u32;
}

// implements duration for each traffic light state
impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 45,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }
}


// Task 2.  Implement a function that sums a collection of u32 integers, with parameter type &[u32] 
// and return type Option. Return None if there is overflow.

// function for calculating sum of a u32 array
fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for num in numbers {
        sum = match sum.checked_add(*num) {
            Some(new_sum) => new_sum,
            None => return None,
        };
    }
    Some(sum)
}

// Task 3. Implement a function that prints the area of a geometric shape. 
// It takes a type that can calculate the area, such as a circle, 
// triangle, or square, as a parameter. This requires the use of generic types and trait bound.

// shape trait that can calculate its area
trait Shape {
    // returns the area of the shape
    fn calculate_area(&self) -> f64;
}

//  circle struct and implement Shape
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// square structure and implement Shape
struct Square {
    side: f64,
}
impl Shape for Square {
    fn calculate_area(&self) -> f64 {
        self.side * self.side
    }
}

// triangle structure and implement Shape
struct Triangle {
    bottom: f64,
    height: f64,
}
impl Shape for Triangle {
    fn calculate_area(&self) -> f64 {
        self.bottom * self.height / 2.0
    }
}

// method main for testing functions
fn main() {
    // traffic lights
    let red = TrafficLight::Red;
    println!("Red light duration: {} seconds", red.duration());

    let yellow = TrafficLight::Yellow;
    println!("Yellow light duration: {} seconds", yellow.duration());

    let green = TrafficLight::Green;
    println!("Green light duration: {} seconds", green.duration());

    // test sum_u32
    let numbers = vec![1, 2, 3, 4, 5];
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("error: overflow"),
    }

    // test Shape
    let circle = Circle { radius: 4.7 };
    println!("Circle area: {:.2} cm^2", circle.calculate_area());

    let square: Square = Square { side: 2.5 };
    println!("Square area: {:.2} cm^2", square.calculate_area());

    let triangle: Triangle = Triangle { bottom: 6.7, height:7.6, };
    println!("Triangle area: {:.2} cm^2", triangle.calculate_area());
}


// test module for all the possible values in TrafficLight
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_durations() {
        assert_eq!(TrafficLight::Red.duration(), 45);
        assert_eq!(TrafficLight::Yellow.duration(), 5);
        assert_eq!(TrafficLight::Green.duration(), 30);
    }

    #[test]
    fn test_sum_u32() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&numbers), Some(15));

        let large_numbers = vec![u32::MAX, 1];
        assert_eq!(sum_u32(&large_numbers), None);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 2.0 };
        assert!((circle.calculate_area() - 12.5664).abs() < 1e-4);
    }
}

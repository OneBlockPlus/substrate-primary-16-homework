/// Enum representing the different states of a traffic light.
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// Trait to get the duration of each traffic light state.
pub trait LightDuration {
    /// Returns the duration of the traffic light state in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// let red_light = TrafficLight::Red;
    /// assert_eq!(red_light.duration(), 60);
    ///
    /// let yellow_light = TrafficLight::Yellow;
    /// assert_eq!(yellow_light.duration(), 5);
    ///
    /// let green_light = TrafficLight::Green;
    /// assert_eq!(green_light.duration(), 30);
    /// ```
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }
}

/// Sums a collection of u32 integers.
/// 
/// # Arguments
/// 
/// * `numbers` - A slice of u32 integers.
/// 
/// # Returns
/// 
/// * `Option<u32>` - The sum of the integers, or None if there is an overflow.
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(sum_u32_collection(&[1, 2, 3]), Some(6));
/// assert_eq!(sum_u32_collection(&[u32::MAX, 1]), None);
/// ```
pub fn sum_u32_collection(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &num| acc.checked_add(num))
}

/// Trait to calculate the area of a shape.
pub trait Area {
    /// Returns the area of the shape.
    ///
    /// # Examples
    ///
    /// ```
    /// let circle = Circle { radius: 3.0 };
    /// assert_eq!(circle.area(), std::f64::consts::PI * 9.0);
    ///
    /// let square = Square { side: 4.0 };
    /// assert_eq!(square.area(), 16.0);
    ///
    /// let triangle = Triangle { base: 4.0, height: 5.0 };
    /// assert_eq!(triangle.area(), 10.0);
    /// ```
    fn area(&self) -> f64;
}

/// Struct representing a circle.
pub struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// Struct representing a square.
pub struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// Struct representing a triangle.
pub struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

/// Prints the area of a shape that implements the Area trait.
/// 
/// # Arguments
/// 
/// * `shape` - A reference to a shape that implements the Area trait.
/// 
/// # Examples
/// 
/// ```
/// let circle = Circle { radius: 3.0 };
/// print_area(&circle);
///
/// let square = Square { side: 4.0 };
/// print_area(&square);
///
/// let triangle = Triangle { base: 4.0, height: 5.0 };
/// print_area(&triangle);
/// ```
pub fn print_area<T: Area>(shape: &T) {
    println!("The area is {}", shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 60);
        assert_eq!(TrafficLight::Yellow.duration(), 5);
        assert_eq!(TrafficLight::Green.duration(), 30);
    }

    #[test]
    fn test_sum_u32_collection() {
        assert_eq!(sum_u32_collection(&[1, 2, 3]), Some(6));
        assert_eq!(sum_u32_collection(&[u32::MAX, 1]), None);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 3.0 };
        assert_eq!(circle.area(), std::f64::consts::PI * 9.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 4.0 };
        assert_eq!(square.area(), 16.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 4.0, height: 5.0 };
        assert_eq!(triangle.area(), 10.0);
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    println!("Red light duration: {} seconds", red_light.duration());

    let numbers = vec![1, 2, 3];
    match sum_u32_collection(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred"),
    }

    let circle = Circle { radius: 3.0 };
    print_area(&circle);

    let square = Square { side: 4.0 };
    print_area(&square);

    let triangle = Triangle { base: 4.0, height: 5.0 };
    print_area(&triangle);
}
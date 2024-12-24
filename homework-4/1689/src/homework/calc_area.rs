pub struct Circle {
    pub radius: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub struct Square {
    pub side: f64,
}

pub trait Area {
    fn calc_area(&self) -> f64;
}

impl Area for Circle {
    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn calc_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Area for Square {
    fn calc_area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn calc_area<T: Area + ?Sized>(shape: &T) -> f64 {
    shape.calc_area()
}

#[test]
fn test_calc_area() {
    struct TestCase {
        input: Box<dyn Area>,
        expected: f64,
    }
    let test_cases = vec![
        TestCase {
            input: Box::new(Circle { radius: 2.0 }),
            expected: 12.566370614359172,
        },
        TestCase {
            input: Box::new(Triangle {
                base: 3.0,
                height: 4.0,
            }),
            expected: 6.0,
        },
        TestCase {
            input: Box::new(Square { side: 5.0 }),
            expected: 25.0,
        },
    ];
    for test_case in test_cases {
        assert_eq!(calc_area(&*test_case.input), test_case.expected);
    }
}

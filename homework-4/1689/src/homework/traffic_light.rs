pub enum TrafficLight {
    Red(u8),
    Yellow(u8),
    Green(u8),
}

pub trait GetDuration {
    fn get_duration(&self) -> u8;
}

impl GetDuration for TrafficLight {
    fn get_duration(&self) -> u8 {
        match self {
            TrafficLight::Red(d) | TrafficLight::Yellow(d) | TrafficLight::Green(d) => *d,
        }
    }
}
#[test]
fn test_traffic_light() {
    struct TestCase {
        input: TrafficLight,
        expected: u8,
    }
    let test_cases = vec![
        TestCase {
            input: TrafficLight::Red(30),
            expected: 30,
        },
        TestCase {
            input: TrafficLight::Yellow(3),
            expected: 3,
        },
        TestCase {
            input: TrafficLight::Green(45),
            expected: 45,
        },
    ];
    for test_case in test_cases {
        assert_eq!(test_case.input.get_duration(), test_case.expected);
    }
}

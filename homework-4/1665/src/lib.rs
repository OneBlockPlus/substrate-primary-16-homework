pub enum TrafficLight {
    Red, Yellow, Green
}

pub trait LightDuration{
    fn get_duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn get_duration(&self) -> u32{
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 30
        }
    }
}

// 计算求和
pub fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32() {
        assert_eq!(sum_u32(&[11,22,33,44]), Some(110));
        assert_eq!(sum_u32(&[u32::MAX, 1]), None);
    }

    #[test]
    fn test_get_duration() {
        assert_eq!(TrafficLight::Red.get_duration(), 60);
        assert_eq!(TrafficLight::Yellow.get_duration(), 10);
        assert_eq!(TrafficLight::Green.get_duration(), 30);
    }
    
}
use std::panic;

pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait Time {
    fn duration(&self) -> i16;
}

impl Time for TrafficLight {
    fn duration(&self) -> i16 {
        match &self {
            TrafficLight::Red => 50,
            TrafficLight::Green => 10,
            TrafficLight::Yellow => 5
        }
    }
}

pub trait Area {
    fn area(&self) -> f32;
}

#[derive(Debug)]
pub struct Rectangle {
    x: f32,
    y: f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        return self.x * self.y;
    }
}

impl Rectangle {
    pub(crate) fn new(x: f32, y: f32) -> Rectangle {
        Rectangle { x, y }
    }
}

pub fn calc_area<T: Area>(t: &T) -> f32 {
    t.area()
}

pub fn vec_sum(elements: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for data in elements {
        match sum.checked_add(*data) {
            None => return None,
            Some(value) => sum = value
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use crate::lesson04::course04::{TrafficLight, Time, vec_sum, Rectangle, calc_area};

    #[test]
    fn should_be_60_seconds_for_red() {
        assert_eq!(50, TrafficLight::Red.duration());
    }

    #[test]
    fn should_be_10_seconds_for_green() {
        assert_eq!(10, TrafficLight::Green.duration());
    }

    #[test]
    fn should_be_5_seconds_for_green() {
        assert_eq!(5, TrafficLight::Yellow.duration());
    }

    #[test]
    fn should_be_normal_when_sum_u32() {
        let elements: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Some(45), vec_sum(&elements));
    }

    #[test]
    fn given_overflow_should_be_none_when_sum_u32() {
        let elements: Vec<u32> = vec![1, std::u32::MAX];
        assert_eq!(None, vec_sum(&elements));
    }

    #[test]
    fn test_calc_area() {
        let rec = Rectangle::new(3.0, 4.0);
        assert_eq!(12.0, calc_area(rec));
    }
}
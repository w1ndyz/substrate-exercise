pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait LightTime {
    fn time(&self) -> u32;
}

impl LightTime for TrafficLight {
    fn time(&self) -> u32 {
        match *self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 60,
        }
    }
}

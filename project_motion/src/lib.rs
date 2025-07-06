#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}
fn round_one_decimal(n: f64) -> f64 {
    (n * 10.0).round() / 10.0
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;
    fn next(&mut self) -> Option<Self::Item> {
        if self.actual_position.y <= 0.0 {
            return None;
        }
        self.time = self.time + 1.0;
        self.actual_velocity.y = self.init_velocity.y - 9.8 * self.time;

        self.actual_position.y = self.init_position.y - (1.0 / 2.0) * 9.8 * self.time * self.time;
        self.actual_velocity.y = (self.actual_velocity.y * 10.0).round() / 10.0;
        self.actual_position.y = (self.actual_position.y * 10.0).round() / 10.0;
        if self.actual_position.y <= 0.0 {
            return None;
        }
        Some(self.clone())
    }
}

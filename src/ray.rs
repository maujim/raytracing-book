use crate::{Point, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

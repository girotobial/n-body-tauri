use crate::{boid::Boid, traits::Intersect, types::BoidRCell, vector::Vector2};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct Boundary {
    pub min: Vector2<f64>,
    pub max: Vector2<f64>,
}

impl Boundary {
    pub const fn new(min: Vector2<f64>, max: Vector2<f64>) -> Self {
        Self { min, max }
    }

    pub fn from_center(center: Vector2<f64>, halfwidth: f64) -> Self {
        let min = Vector2::new(center.x - halfwidth, center.y - halfwidth);
        let max = Vector2::new(center.x + halfwidth, center.y + halfwidth);
        Self { min, max }
    }

    pub fn center(&self) -> Vector2<f64> {
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;

        let center_x = self.min.x + width / 2.0;
        let center_y = self.min.y + height / 2.0;

        Vector2::new(center_x, center_y)
    }

    pub fn half_size(&self) -> f64 {
        (self.max.x - self.min.x) / 2.0
    }
}

impl Intersect<Vector2<f64>> for Boundary {
    fn intersects(&self, other: &Vector2<f64>) -> bool {
        self.min.x <= other.x
            && self.min.y <= other.y
            && self.max.x >= other.x
            && self.max.y >= other.y
    }
}

impl Intersect<Self> for Boundary {
    fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.max.x >= other.min.x
            && self.max.y >= other.min.y
    }
}

impl Intersect<BoidRCell> for Boundary {
    fn intersects(&self, other: &BoidRCell) -> bool {
        self.intersects(&other.position())
    }
}
impl Intersect<&Boid> for Boundary {
    fn intersects(&self, other: &&Boid) -> bool {
        self.intersects(&other.position())
    }
}

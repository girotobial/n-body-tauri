use crate::vector::Vector2;

pub trait Intersect<T> {
    fn intersects(&self, other: &T) -> bool;
}

pub trait Mass {
    fn mass(&self) -> f64;
    fn center_of_mass(&self) -> Vector2<f64>;
}

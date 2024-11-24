use crate::{vector::Vector2, Boid, Boundary};
use std::sync::Arc;

#[derive(serde::Serialize, Clone, Copy)]
pub struct Body {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    mass: f64,
    radius: f64,
}

impl From<&Boid> for Body {
    fn from(value: &Boid) -> Self {
        let lock = value.inner.read().expect("RWLock was poisoned");
        Self {
            position: lock.pos,
            velocity: lock.velocity,
            mass: lock.mass,
            radius: lock.radius(),
        }
    }
}
impl From<&Arc<Boid>> for Body {
    fn from(value: &Arc<Boid>) -> Self {
        Self {
            position: value.position(),
            velocity: value.velocity(),
            mass: value.mass(),
            radius: value.radius(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct TreeState {
    pub boundaries: Vec<Boundary>,
    pub center_of_mass: Vector2<f64>,
    pub outer_bounds: Boundary,
    pub center: Vector2<f64>,
}

impl TreeState {
    pub fn new(
        boundaries: Vec<Boundary>,
        center_of_mass: Vector2<f64>,
        outer_bounds: Boundary,
        center: Vector2<f64>,
    ) -> Self {
        Self {
            boundaries,
            center_of_mass,
            outer_bounds,
            center,
        }
    }
}

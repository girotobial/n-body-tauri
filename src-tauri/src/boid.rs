use std::{fmt, sync::RwLock};

use crate::{traits::Mass, vector::Vector2};

#[derive(Debug, Clone, Copy)]
struct BoidInner {
    pub pos: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub mass: f64,
}

impl BoidInner {
    pub fn new(x: f64, y: f64, mass: f64) -> Self {
        Self {
            pos: Vector2::new(x, y),
            velocity: Vector2::default(),
            mass,
        }
    }
}

#[derive(Debug)]
pub struct Boid {
    inner: RwLock<BoidInner>,
}

impl Boid {
    pub fn new(x: f64, y: f64, mass: f64) -> Self {
        Self {
            inner: RwLock::new(BoidInner::new(x, y, mass)),
        }
    }

    pub fn position(&self) -> Vector2<f64> {
        self.inner.read().expect("RWLock was poisoned").pos
    }

    pub fn mass(&self) -> f64 {
        self.inner.read().expect("RWLock was poisoned").mass
    }

    pub fn velocity(&self) -> Vector2<f64> {
        self.inner.read().expect("RWLock was poisoned").velocity
    }

    pub fn set_position(&self, position: Vector2<f64>) {
        self.inner.write().expect("RWLock was poisoned").pos = position;
    }

    pub fn set_velocity(&self, velocity: Vector2<f64>) {
        self.inner.write().expect("RWLock was poisoned").velocity = velocity;
    }
}

impl Mass for Boid {
    fn mass(&self) -> f64 {
        self.mass()
    }

    fn center_of_mass(&self) -> Vector2<f64> {
        self.position()
    }
}

impl fmt::Display for Boid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Body at {} with mass {:.2}",
            self.position(),
            self.mass()
        )
    }
}

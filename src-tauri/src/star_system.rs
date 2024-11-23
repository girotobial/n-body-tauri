use crate::{boid::Boid, Vector2};

pub struct StarSystem;

impl StarSystem {
    pub fn generate_disk_system(n: usize, radius: f64) -> Vec<Boid> {
        let mut bodies = Vec::new();
        for i in 0..n {
            let theta = i as f64 * 2.0 * std::f64::consts::PI / n as f64;
            let position = [radius * theta.cos(), radius * theta.sin()];
            let velocity = Vector2::new(-theta.sin(), theta.cos()); // Circular orbit velocities
            let body = Boid::new(position[0], position[1], 1.0 / n as f64);
            body.set_velocity(velocity);
            bodies.push(body)
        }
        bodies
    }
}

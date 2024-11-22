// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![warn(clippy::perf, clippy::pedantic)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use boid::Boid;
use boundary::Boundary;
use quadtree::Quadtree;
use tauri::State;
use types::BoidRCell;
use vector::Vector2;

mod boid;
mod boundary;
mod quadtree;
mod traits;
mod types;
mod vector;

const TIMESTEP: u64 = 10;
const DT: f64 = TIMESTEP as f64 / 1000.0;
const MASS_ONE: f64 = 125e12;
const MASS_TWO: f64 = 10e11;
const SATELITE_MASS: f64 = 10e9;
pub const GRAVITY: f64 = 6.67430e-11;
const THETA: f64 = 0.5;
const CENTER_X: f64 = 250.0;
const CENTER_Y: f64 = 250.0;

static BOIDS: RwLock<Vec<BoidRCell>> = RwLock::new(Vec::new());
static TREE_STATE: RwLock<Option<TreeState>> = RwLock::new(Option::None);
static MIN: Mutex<Vector2<f64>> = Mutex::new(Vector2::new(0.0, 0.0));
static MAX: Mutex<Vector2<f64>> = Mutex::new(Vector2::new(1000.0, 1000.0));

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
struct TreeState {
    boundaries: Vec<Boundary>,
    center_of_mass: Vector2<f64>,
}

#[tauri::command]
fn get_bodies(boids: State<&'static RwLock<Vec<BoidRCell>>>) -> Vec<Body> {
    boids.read().unwrap().iter().map(Body::from).collect()
}

#[tauri::command]
fn get_tree(tree_state: State<&'static RwLock<Option<TreeState>>>) -> Option<TreeState> {
    tree_state.read().unwrap().clone()
}

fn cold_colapse(center: Vector2<f64>, radius: f64, count: usize) -> Vec<Arc<Boid>> {
    let increment = std::f64::consts::PI * 2.0 / count as f64;
    let mut boids = vec![];
    let mut theta = 0.0;

    while theta < std::f64::consts::PI * 2.0 {
        let x = theta.sin() * radius + center.x;
        let y = theta.cos() * radius + center.y;
        boids.push(Arc::new(Boid::new(x, y, MASS_ONE)));
        theta += increment;
    }
    boids
}

fn orbital_speed(radius: f64, mass: f64) -> f64 {
    (GRAVITY * mass / radius).sqrt()
}

fn main() {
    let mass_one = {
        let boid = Boid::new(CENTER_X, CENTER_Y, MASS_ONE);
        boid.set_velocity(Vector2::new(0.0, 0.0));
        Arc::new(boid)
    };
    let mass_two_speed = (GRAVITY * MASS_ONE / 100.0).sqrt();
    let mass_two = {
        let boid = Boid::new(CENTER_X + 100.0, CENTER_Y, MASS_TWO);
        boid.set_velocity(Vector2::new(0.0, mass_two_speed));
        Arc::new(boid)
    };
    let mass_three = {
        let boid = Boid::new(CENTER_X - 100.0, CENTER_Y, MASS_TWO);
        boid.set_velocity(Vector2::new(0.0, -mass_two_speed));
        Arc::new(boid)
    };
    let mass_four = {
        let boid = Boid::new(CENTER_X + 50.0, CENTER_Y + 86.6, SATELITE_MASS);
        boid.set_velocity(Vector2::new(-7.94, 4.58));
        Arc::new(boid)
    };

    let mass_five = {
        let boid = Boid::new(CENTER_X + 50.0, CENTER_Y - 86.6, SATELITE_MASS);
        boid.set_velocity(Vector2::new(7.94, 4.58));
        Arc::new(boid)
    };

    let moon_sun_speed = orbital_speed(110.0, MASS_ONE);
    let moon_speed = orbital_speed(10.0, MASS_TWO);

    let mass_six = {
        let boid = Boid::new(CENTER_X + 110.0, CENTER_Y, SATELITE_MASS);
        boid.set_velocity(Vector2::new(0.0, moon_sun_speed + moon_speed));
        Arc::new(boid)
    };

    {
        let mut lock = BOIDS.write().unwrap();
        for boid in [
            mass_one, mass_two, mass_three, mass_four, mass_five, mass_six,
        ] {
            lock.push(boid);
        }
    }

    tauri::Builder::default()
        .setup(|_| {
            std::thread::spawn(|| loop {
                let mut min = MIN.lock().unwrap();
                let mut max = MAX.lock().unwrap();

                let mut tree = Quadtree::new(boundary::Boundary::new(*min, *max));

                for body in BOIDS.read().unwrap().iter() {
                    if let Ok(()) = tree.insert(body.clone()) {}
                }

                {
                    use std::mem::replace;
                    let mut tree_state = TREE_STATE.write().expect("Could not acquire bounds lock");
                    let new_state = TreeState {
                        boundaries: tree.boundaries(),
                        center_of_mass: tree.center_of_mass(),
                    };
                    let _ = replace(&mut *tree_state, Some(new_state));
                }
                for body in BOIDS.write().unwrap().iter() {
                    let force = tree.calculate_force(body, THETA);
                    let acceleration = force * (1.0 / body.mass());
                    let new_velocity = body.velocity() + acceleration * DT;
                    let new_position = body.position() + new_velocity * DT;
                    body.set_velocity(new_velocity);
                    body.set_position(new_position);

                    // Resize the tree
                    if new_position.x < min.x {
                        min.x = new_position.x;
                    }
                    if new_position.y < min.y {
                        min.y = new_position.y;
                    }
                    if new_position.x > max.x {
                        max.x = new_position.x;
                    }
                    if new_position.y > max.y {
                        max.y = new_position.y;
                    }
                }
                std::thread::sleep(Duration::from_millis(TIMESTEP));
            });
            Ok(())
        })
        .manage(&BOIDS)
        .manage(&TREE_STATE)
        .invoke_handler(tauri::generate_handler![get_bodies, get_tree])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

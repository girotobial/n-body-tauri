// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![warn(clippy::perf, clippy::nursery, clippy::pedantic)]
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
const MASS_TWO: f64 = 4_833_385_275_574.0;
const STANDARD_G: f64 = 10000.0;
pub const GRAVITY: f64 = STANDARD_G / (MASS_ONE + MASS_TWO);
const THETA: f64 = 0.5;

static BOIDS: RwLock<Vec<BoidRCell>> = RwLock::new(Vec::new());
static BOUNDS: RwLock<Vec<Boundary>> = RwLock::new(Vec::new());
static MAX: Mutex<Vector2<f64>> = Mutex::new(Vector2::new(0.0, 0.0));
static MIN: Mutex<Vector2<f64>> = Mutex::new(Vector2::new(1000.0, 1000.0));
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(serde::Serialize, Clone, Copy)]
pub struct Body {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    mass: f64,
}

impl From<&Boid> for Body {
    fn from(value: &Boid) -> Self {
        Self {
            position: value.position(),
            velocity: value.velocity(),
            mass: value.mass(),
        }
    }
}
impl From<&Arc<Boid>> for Body {
    fn from(value: &Arc<Boid>) -> Self {
        Self {
            position: value.position(),
            velocity: value.velocity(),
            mass: value.mass(),
        }
    }
}

#[tauri::command]
fn get_bodies(boids: State<&'static RwLock<Vec<BoidRCell>>>) -> Vec<Body> {
    boids.read().unwrap().iter().map(Body::from).collect()
}

#[tauri::command]
fn get_boundaries(bounds: State<&'static RwLock<Vec<Boundary>>>) -> Vec<Boundary> {
    bounds.read().unwrap().iter().copied().collect()
}

fn main() {
    let mass_one = {
        let boid = Boid::new(200.0, 250.0, MASS_ONE);
        boid.set_velocity(Vector2::new(0.0, 1.0));
        Arc::new(boid)
    };
    let mass_two = {
        let boid = Boid::new(200.0, 150.0, MASS_TWO);
        boid.set_velocity(Vector2::new(10.0, 1.0));
        Arc::new(boid)
    };
    let mass_three = {
        let boid = Boid::new(500.0, 250.0, MASS_ONE);
        boid.set_velocity(Vector2::new(0.0, -1.0));
        Arc::new(boid)
    };
    let mass_four = {
        let boid = Boid::new(500.0, 150.0, MASS_TWO);
        boid.set_velocity(Vector2::new(-10.0, -1.0));
        Arc::new(boid)
    };

    let mass_five = {
        let boid = Boid::new(300.0, 0.0, MASS_TWO);
        boid.set_velocity(Vector2::new(0.0, 10.0));
        Arc::new(boid)
    };
    let mass_six = {
        let boid = Boid::new(0.0, 200.0, MASS_TWO);
        boid.set_velocity(Vector2::new(10.0, 0.0));
        Arc::new(boid)
    };

    for boid in [
        mass_one, mass_two, mass_three, mass_four, mass_five, mass_six,
    ] {
        BOIDS.write().unwrap().push(boid);
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
                    let mut bounds = BOUNDS.write().expect("Could not acquire bounds lock");
                    let _ = replace(&mut *bounds, tree.boundaries());
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
        .manage(&BOUNDS)
        .invoke_handler(tauri::generate_handler![get_bodies, get_boundaries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

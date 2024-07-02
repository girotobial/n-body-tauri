use crate::{
    boid::Boid,
    boundary::Boundary,
    traits::{Intersect, Mass},
    types::BoidRCell,
    vector::Vector2,
    Body, GRAVITY,
};

type Child = Box<Node>;

type InsertionResult = Result<(), InsertionError>;

#[derive(thiserror::Error, Debug)]
pub enum InsertionError {
    #[error("The body at {position:?} cannot fit inside {boundary:?}")]
    OutOfBounds {
        boundary: Boundary,
        position: Vector2<f64>,
    },
}

#[derive(Debug, Clone)]
enum Contents {
    Empty,
    Boid(BoidRCell),
    Children([Child; 4]),
}

#[derive(Debug, Clone)]
struct Node {
    boundary: Boundary,
    center_of_mass: Vector2<f64>,
    mass: f64,
    contents: Contents,
}

impl Node {
    pub fn new(boundary: Boundary) -> Self {
        Self {
            boundary,
            center_of_mass: boundary.center(),
            mass: 0.0,
            contents: Contents::Empty,
        }
    }

    pub fn insert(&mut self, boid: BoidRCell) -> InsertionResult {
        if !self.boundary.intersects(&boid) {
            return Err(InsertionError::OutOfBounds {
                boundary: self.boundary,
                position: boid.position(),
            });
        }

        match &mut self.contents {
            Contents::Empty => {
                self.update_com(&boid);
                self.contents = Contents::Boid(boid);
                return Ok(());
            }
            Contents::Boid(current_boid) => {
                let per_new_part = (self.boundary.max - self.boundary.min) / 2.0;
                let boundary_min = self.boundary.min;

                let new_children: [Child; 4] = std::array::from_fn(|i| {
                    let (x, y) = match i {
                        0 => Some((0, 0)),
                        1 => Some((0, 1)),
                        2 => Some((1, 0)),
                        3 => Some((1, 1)),
                        _ => None,
                    }
                    .expect("Somehow the integer has exceeded 3");
                    let offset = Vector2::new(x as f64 * per_new_part.x, y as f64 * per_new_part.y);
                    let min = boundary_min + offset;
                    let max = min + per_new_part;
                    Box::new(Node::new(Boundary { min, max }))
                });
                let old_boid = current_boid.clone();
                self.contents = Contents::Children(new_children);
                match self.insert(old_boid.clone()) {
                    Ok(_) => (),
                    Err(e) => {
                        self.contents = Contents::Boid(old_boid);
                        return Err(e);
                    }
                }
                match self.insert(boid.clone()) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                self.update_com(&boid);
                return Ok(());
            }
            Contents::Children(children) => {
                for child in children.iter_mut() {
                    match child.insert(boid.clone()) {
                        Ok(_) => (),
                        Err(_) => continue,
                    }
                }
                self.update_com(&boid);
                return Ok(());
            }
        }
    }

    fn update_com(&mut self, boid: &Boid) {
        let new_mass = self.mass + boid.mass();
        let new_com = (self.center_of_mass * (self.mass as f64)
            + boid.center_of_mass() * (boid.mass() as f64))
            / (new_mass as f64);
        self.mass = new_mass;
        self.center_of_mass = new_com;
    }

    pub fn calculate_force(&self, body: &BoidRCell, theta: f64) -> Vector2<f64> {
        if let Contents::Empty = self.contents {
            return Vector2::default();
        }

        let d = self.boundary.half_size() * 2.0;
        let r = (body.position() - self.center_of_mass).magnitude();

        if d / r < theta {
            if r == 0.0 {
                return Vector2::default();
            }
            let direction = self.center_of_mass - body.position();
            let force_magnitude = (GRAVITY * self.mass * body.mass()) / (r * r);
            return direction * force_magnitude / r;
        }
        match &self.contents {
            Contents::Boid(body2) => {
                let direction = body2.position() - body.position();
                let distance = direction.magnitude();
                if distance == 0.0 {
                    return Vector2::default();
                }
                let force_magnitude = (GRAVITY * self.mass * body.mass()) / (distance * distance);
                return direction * force_magnitude / distance;
            }
            Contents::Children(children) => {
                let mut force = Vector2::default();
                for child in children {
                    force = force + child.calculate_force(body, theta);
                }
                force
            }
            Contents::Empty => return Vector2::default(),
        }
    }

    pub fn boundaries(&self) -> Vec<Boundary> {
        if let Contents::Children(children) = &self.contents {
            children.iter().flat_map(|c| c.boundaries()).collect()
        } else {
            vec![self.boundary]
        }
    }
}
#[derive(Debug)]
pub struct Quadtree {
    head: Node,
    boids: Vec<BoidRCell>,
}

impl Quadtree {
    pub fn new(boundary: Boundary) -> Self {
        Self {
            head: Node::new(boundary),
            boids: Vec::new(),
        }
    }

    pub fn insert(&mut self, boid: BoidRCell) -> InsertionResult {
        self.head.insert(boid.clone())?;
        self.boids.push(boid);
        Ok(())
    }

    pub fn bodies(&self) -> Vec<Body> {
        self.boids
            .iter()
            .map(|b| {
                let position = b.position();
                let velocity = b.velocity();
                let mass = b.mass();
                Body {
                    position,
                    velocity,
                    mass,
                }
            })
            .collect()
    }

    pub fn calculate_force(&self, body: &BoidRCell, theta: f64) -> Vector2<f64> {
        self.head.calculate_force(body, theta)
    }

    pub fn boundaries(&self) -> Vec<Boundary> {
        self.head.boundaries()
    }
    pub fn boundary(&self) -> Boundary {
        self.head.boundary
    }
    pub fn center_of_mass(&self) -> Vector2<f64> {
        self.head.center_of_mass
    }
}

use std::vec::Vec;
use std::cell::RefCell;
use std::rc::Rc;

use crate::constants::{SCALE, TIMESTEP, G};

use na::{Point2, Vector2};

pub struct Body {
    pub mass: f64, // kilograms
    pub radius: f64, // pixels (int?)
    pub position: Point2<f64>, // position over 0..SCALE
    pub velocity: Vector2<f64>, // 
}

impl Body {
    pub fn new () -> Body {
        Body {
            mass: 0.0, 
            radius: 0.0, 
            position: Point2::new(0.0, 0.0),
            velocity: Vector2::zeros(),
        }
    }
}

pub struct BodyBuilder {
    body: Body,
    dest: Rc<RefCell<Vec<Body>>>,
}

impl BodyBuilder {
    pub fn new(set: Rc<RefCell<Vec<Body>>>) -> BodyBuilder {
        BodyBuilder { body: Body::new(), dest: set }
    }

    pub fn with_mass(mut self, mass: f64) -> BodyBuilder {
        self.body.mass = mass;
        self
    }

    pub fn with_radius(mut self, radius: f64) -> BodyBuilder {
        self.body.radius = radius;
        self
    }

    pub fn with_position(mut self, pos: Point2<f64>) -> BodyBuilder {
        self.body.position = pos;
        self
    }

    pub fn with_velocity(mut self, vel: Vector2<f64>) -> BodyBuilder {
        self.body.velocity = vel;
        self
    }

    pub fn commit(self) -> usize {
        self.dest.borrow_mut().push(self.body);
        self.dest.borrow().len() - 1
    }

}

pub struct System {
    bodies: Rc<RefCell<Vec<Body>>>,
}

impl System {
    pub fn new() -> System {
        System { bodies: Rc::new(RefCell::new(Vec::new())) }
    }

    pub fn add_body(&self) -> BodyBuilder {
        BodyBuilder::new(self.bodies.clone())
    }

    pub fn bodies(&self) -> Rc<RefCell<Vec<Body>>> {
        self.bodies.clone()
    }

    pub fn tick(&self) {
        let mut final_forces: Vec<Vector2<f64>> = vec![];
        for (idx, body) in self.bodies.borrow().iter().enumerate() {
            let final_force = self.bodies.borrow()
                .iter().enumerate()
                .filter(|(i, _)| *i != idx)
                .fold(Vector2::zeros(), |acc, (i, other)| {
                    let distance = na::distance(&body.position, &other.position);
                    let force = G * (body.mass * other.mass) / distance.powf(2.0);

                    // get the angle here
                    let v = other.position - body.position;
                    let mut dir = Vector2::new(v.x, v.y).normalize();
                    dir *= force;

                    acc + dir
                });
            final_forces.push(final_force);
        }

        for (body, force) in self.bodies.borrow_mut().iter_mut().zip(final_forces.iter()) {
            body.velocity += force / body.mass * TIMESTEP;
            body.position += body.velocity * TIMESTEP;
        }
    }
}
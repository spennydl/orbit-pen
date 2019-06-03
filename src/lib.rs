mod system;
mod utils;
mod constants;

extern crate nalgebra as na;
extern crate web_sys;

use na::{Vector2, Point2};
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use system::System;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub struct Simulation {
    canvas: web_sys::HtmlCanvasElement,
    sys: System,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new() -> Simulation {
        utils::set_panic_hook();

        let document = web_sys::window()
            .expect("there should be a window")
            .document()
            .expect("there should also be a document");
        let canvas = document
            .get_element_by_id("sim-canvas")
            .expect("the canvas should exist");
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let sys = System::new();

        Simulation { canvas, sys }
    }

    pub fn add_body(&self, mass: f64, radius: f64, 
                        x: f64, y: f64,
                        vx: f64, vy: f64) -> f64 {
        self.sys.add_body()
            .with_mass(mass)
            .with_radius(radius)
            .with_position(Point2::new(x * constants::UNIT, y * constants::UNIT))
            .with_velocity(Vector2::new(vx, vy))
            .commit() as f64
    }

    pub fn start(self) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let ctx = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let center = Vector2::new(1280.0 / 2.0, 720.0 / 2.0);
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            self.sys.tick();

            let black = JsValue::from_str("#000000");
            let body_color = JsValue::from_str("#EAEAEA");

            ctx.set_fill_style(&black);
            ctx.fill_rect(0.0, 0.0, 1280.0, 720.0);

            ctx.set_fill_style(&body_color);
            for body in self.sys.bodies().borrow().iter() {
                ctx.begin_path();
                ctx.arc(
                    (body.position[0] * constants::SCALE) + center[0],
                    (body.position[1] * constants::SCALE) + center[1],
                    body.radius,
                    0.0,
                    2.0 * constants::PI,
                )
                .unwrap();
                ctx.fill();
            }
            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
}

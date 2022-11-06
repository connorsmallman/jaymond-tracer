mod tuple;
mod canvas;

use std::fs;
use std::thread::current;
use crate::tuple::tuple::{Color, Tuple};
use crate::canvas::canvas::Canvas;

#[derive(Debug)]
struct World {
    gravity: Tuple,
    wind: Tuple,
}

impl World {
    fn new(gravity: Tuple, wind: Tuple) -> World {
        World { gravity, wind }
    }
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Projectile {
        Projectile { position, velocity }
    }
}

impl Projectile {
    pub fn tick(&self, world: &World) -> Projectile {
        Projectile {
            position: self.position + self.velocity,
            velocity: self.velocity + world.gravity + world.wind,
        }
    }
}

fn tick(environment: World, projectile: Projectile) -> Projectile {
    projectile.tick(&environment)
}

enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds,
}

impl Pixel {
    pub fn from_point_for_canvas(point: Tuple, canvas: &Canvas) -> Pixel {
        let rx = point.x.round();
        let ry = point.y.round();

        let ux = rx as usize;
        let uy = ry as usize;

        if rx.is_sign_negative() || ry.is_sign_negative() || ux > canvas.width || uy > canvas.height {
            return Pixel::OutOfBounds;
        }

        let screen_x = ux;
        let screen_y = canvas.height - uy;

        Pixel::Coordinate {
            x: screen_x,
            y: screen_y,
        }
    }
}

fn main() {
    let environment = World::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.02, 0.0, 0.0),
    );
    let mut projectile = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    );

    let mut canvas = Canvas::new(900, 500);
    let color = Color::new(1.0, 1.0, 0.0);

    println!("{:?}", environment);

    let mut iteration: i32 = 0;
    while projectile.position.y > 0.0 {
        println!("{}: {:?}", iteration, projectile);

        match Pixel::from_point_for_canvas(projectile.position, &canvas) {
            Pixel::Coordinate { x, y } => {
                canvas.write_pixel(x, y, color);
            }
            Pixel::OutOfBounds => {}
        }

        projectile = projectile.tick(&environment);
        iteration += 1;
    }
    println!("FINISHED => {}: {:?}", iteration, projectile);

    println!("Writing ./output.ppm");
    let ppm = canvas.to_ppm();
    fs::write("./output.ppm", ppm).expect("Could not write ouput.ppm to disk.");
    println!("Everything done.");
}
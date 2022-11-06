mod tuple;
mod canvas;

use crate::tuple::tuple::Tuple;

fn main() {
    let world = World::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(2.0, 0.0, 0.0),
    );
    let projectile = Projectile::new(
        Tuple::point(0.0, 5.0, 0.0),
        Tuple::vector(0.02, 0.0, 0.0)
    );
    let mut iterations = 0;
    let mut current = projectile;
    while current.position.y > 0.0 {
        current = current.tick(&world);
        println!("{}: {:?}", iterations, current);
        iterations += 1;
    }
}

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
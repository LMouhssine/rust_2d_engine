// src/systems/movement.rs

use specs::{System, ReadStorage, WriteStorage, Join};
use crate::components::{Position, Velocity};

pub struct MovementSystem; // Doit être public

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        specs::Read<'a, f32>,
    );

    fn run(&mut self, (mut positions, velocities, delta_time): Self::SystemData) {
        for (pos, vel) in (&mut positions, &velocities).join() {
            pos.x += vel.x * *delta_time;
            pos.y += vel.y * *delta_time;
            
            // Garder dans les limites de l'écran
            pos.x = pos.x.clamp(0.0, 800.0);
            pos.y = pos.y.clamp(0.0, 600.0);
        }
    }
}

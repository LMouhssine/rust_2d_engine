use specs::{System, ReadStorage, WriteStorage, Join};
use crate::components::{Position, Velocity, DeltaTime};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        specs::Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut positions, velocities, delta_time): Self::SystemData) {
        for (pos, vel) in (&mut positions, &velocities).join() {
            pos.x += vel.x * delta_time.0;
            pos.y += vel.y * delta_time.0;
            
            // Garder dans les limites de l'écran
            pos.x = pos.x.clamp(0.0, 800.0);
            pos.y = pos.y.clamp(0.0, 600.0);
        }
    }
} 
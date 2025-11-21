use specs::{System, ReadStorage, WriteStorage, Join, Read};
use crate::components::{Position, Velocity, Gravity, Grounded};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Grounded>,
        Read<'a, f32>,
    );

    fn run(&mut self, (mut positions, mut velocities, gravity, _grounded, delta_time): Self::SystemData) {
        let dt = *delta_time;
        
        // Apply Gravity
        for (vel, _grav) in (&mut velocities, &gravity).join() {
            vel.y += 980.0 * dt; // Gravity constant
        }

        // Apply Velocity to Position
        for (pos, vel) in (&mut positions, &mut velocities).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
            
            // Apply friction/drag on X axis
            vel.x *= 0.90; 
            if vel.x.abs() < 1.0 { vel.x = 0.0; }

            // Screen bounds (only X)
            pos.x = pos.x.clamp(0.0, 750.0); // 800 - width approx
        }
    }
}

use specs::{System, ReadStorage, WriteStorage, Join, Read, Entities};
use crate::components::{Position, Velocity, Gravity, Grounded};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Grounded>,
        Read<'a, f32>,
    );

    fn run(&mut self, (entities, mut positions, mut velocities, gravity, grounded, delta_time): Self::SystemData) {
        let dt = *delta_time;
        
        for (entity, vel, _grav) in (&entities, &mut velocities, &gravity).join() {
            let is_grounded = grounded.get(entity).is_some();
            
            // Apply Gravity
            if !is_grounded {
                vel.y += 1500.0 * dt; // Gravity constant
            } else if vel.y > 0.0 {
                vel.y = 0.0;
            }

            // Apply friction/drag on X axis
            let friction: f32 = if is_grounded { 0.85 } else { 0.95 };
            vel.x *= friction.powf(dt * 60.0); 
            if vel.x.abs() < 5.0 { vel.x = 0.0; }
        }

        // Apply Velocity to Position
        for (pos, vel) in (&mut positions, &mut velocities).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
            
            // Screen bounds
            pos.x = pos.x.clamp(0.0, 760.0); 
        }
    }
}

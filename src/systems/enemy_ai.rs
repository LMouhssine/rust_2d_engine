use specs::{System, WriteStorage, ReadStorage, Join, Read, Entities};
use crate::components::{Position, Velocity, Enemy, Platform, Renderable};

pub struct EnemyAISystem;

impl<'a> System<'a> for EnemyAISystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Platform>,
        ReadStorage<'a, Renderable>,
        Read<'a, f32>,
    );

    fn run(&mut self, (_entities, mut velocities, positions, enemies, _platforms, renderables, delta_time): Self::SystemData) {
        let dt = *delta_time;

        for (_enemy, vel, pos, _render) in (&enemies, &mut velocities, &positions, &renderables).join() {
            // Simple Patrol: Move back and forth, check for "edges" or "walls"
            // For now, just reverse if hitting a "wall" boundary
            if pos.x <= 0.0 || pos.x >= 760.0 {
                vel.x = -vel.x;
            }

            // More advanced: reverse if no platform below? 
            // Simplified for now.
        }
    }
}

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

    fn run(&mut self, (_entities, mut velocities, positions, enemies, platforms, renderables, delta_time): Self::SystemData) {
        let dt = *delta_time;

        for (_enemy, vel, pos, render) in (&enemies, &mut velocities, &positions, &renderables).join() {
            // Simple Patrol: Move back and forth, check for "edges" or "walls"
            if pos.x <= 0.0 || pos.x >= 760.0 {
                vel.x = -vel.x;
            } else {
                // Edge Detection: Check if there's a platform below the next position
                let next_x = pos.x + vel.x * dt * 5.0; // Check a bit ahead
                let mut has_ground_ahead = false;
                for (plat_pos, plat_render, _plat) in (&positions, &renderables, &platforms).join() {
                    if next_x + render.width > plat_pos.x && next_x < plat_pos.x + plat_render.width {
                        if (plat_pos.y - (pos.y + render.height)).abs() < 10.0 {
                            has_ground_ahead = true;
                            break;
                        }
                    }
                }
                if !has_ground_ahead {
                    vel.x = -vel.x;
                }
            }
        }
    }
}

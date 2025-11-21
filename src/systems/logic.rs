use specs::{System, WriteStorage, Join};
use crate::components::{Position, Player, Velocity};

pub struct LogicSystem;

impl<'a> System<'a> for LogicSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, (mut positions, mut velocities, mut players): Self::SystemData) {
        for (pos, vel, player) in (&mut positions, &mut velocities, &mut players).join() {
            // Check Fall (Death)
            if pos.y > 700.0 {
                // Reset
                pos.x = 100.0;
                pos.y = 100.0;
                vel.x = 0.0;
                vel.y = 0.0;
                player.score = 0;
                println!("Game Over! Resetting...");
            }

            // Check Win (Simple console log for now, visual handled in render)
            if player.score >= 50 { 
                 // Keep score capped or just let it be
            }
        }
    }
}

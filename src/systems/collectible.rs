use specs::{System, ReadStorage, WriteStorage, Entities, Join};
use crate::components::{Position, Collidable, Collectible, Player};

pub struct CollectibleSystem;

impl<'a> System<'a> for CollectibleSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collidable>,
        ReadStorage<'a, Collectible>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, (entities, positions, collidables, collectibles, mut players): Self::SystemData) {
        for (entity, pos, col, _) in (&entities, &positions, &collidables, &collectibles).join() {
            for (player_pos, player_col, player) in (&positions, &collidables, &mut players).join() {
                let dx = pos.x - player_pos.x;
                let dy = pos.y - player_pos.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < (col.radius + player_col.radius) {
                    // Collectible récupéré
                    let _ = entities.delete(entity);
                    player.score += 1;
                    println!("Score: {}", player.score);
                }
            }
        }
    }
}
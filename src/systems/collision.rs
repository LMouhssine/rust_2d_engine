use specs::{System, ReadStorage, WriteStorage, Join, Entities};
use crate::components::{Position, Collidable, Collectible, Platform, Velocity, Grounded, Player};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>, // Changed to WriteStorage
        ReadStorage<'a, Collidable>,
        ReadStorage<'a, Collectible>,
        ReadStorage<'a, Platform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Grounded>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, (entities, mut positions, collidables, collectibles, platforms, mut velocities, mut grounded, mut players): Self::SystemData) {
        grounded.clear();

        // We need to collect collisions first to avoid double borrow issues if we were to try complex things,
        // but actually, if we iterate over entities and then query others, we might be okay if we don't borrow `positions` for the whole loop?
        // No, `join` borrows the storage.
        
        // Strategy: Collect platform collisions (Entity, NewY)
        let mut platform_collisions = Vec::new();
        
        // We can iterate immutably first
        for (entity, pos, vel, _player, _collider) in (&entities, &positions, &velocities, &players, &collidables).join() {
            let player_rect = (pos.x, pos.y, 50.0, 50.0);

            for (_platform_entity, platform_pos, _platform, platform_collider) in (&entities, &positions, &platforms, &collidables).join() {
                let platform_rect = (platform_pos.x, platform_pos.y, platform_collider.radius * 2.0, 20.0);

                if check_aabb(player_rect, platform_rect) {
                    // If falling and hitting top
                    if vel.y > 0.0 && pos.y + 50.0 <= platform_pos.y + 15.0 {
                        platform_collisions.push((entity, platform_pos.y - 50.0));
                    }
                }
            }
        }

        // Apply platform collisions
        for (entity, new_y) in platform_collisions {
            if let Some(pos) = positions.get_mut(entity) {
                pos.y = new_y;
            }
            if let Some(vel) = velocities.get_mut(entity) {
                vel.y = 0.0;
            }
            grounded.insert(entity, Grounded).ok();
        }

        // Collectibles (same logic)
        let mut to_remove = Vec::new();
        let mut score_updates = Vec::new();

        for (player_entity, player_pos, _player, player_collider) in (&entities, &positions, &players, &collidables).join() {
            for (collectible_entity, collectible_pos, _collectible, collectible_collider) in (&entities, &positions, &collectibles, &collidables).join() {
                let distance = ((player_pos.x - collectible_pos.x).powi(2) + (player_pos.y - collectible_pos.y).powi(2)).sqrt();
                if distance < (player_collider.radius + collectible_collider.radius) {
                    to_remove.push(collectible_entity);
                    score_updates.push((player_entity, 10));
                }
            }
        }

        for e in to_remove {
            entities.delete(e).unwrap();
        }
        
        for (entity, score) in score_updates {
            if let Some(p) = players.get_mut(entity) {
                p.score += score;
                // println!("Score: {}", p.score);
            }
        }
    }
}

fn check_aabb(r1: (f32, f32, f32, f32), r2: (f32, f32, f32, f32)) -> bool {
    r1.0 < r2.0 + r2.2 &&
    r1.0 + r1.2 > r2.0 &&
    r1.1 < r2.1 + r2.3 &&
    r1.1 + r1.3 > r2.1
}
use specs::{System, ReadStorage, WriteStorage, Join, Entities};
use crate::components::{Position, Collidable, Collectible, Platform, Velocity, Grounded, Player, Enemy, Renderable, Health};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>, 
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Collidable>,
        ReadStorage<'a, Collectible>,
        ReadStorage<'a, Platform>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Grounded>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Health>,
    );

    fn run(&mut self, (entities, mut positions, renderables, _collidables, collectibles, platforms, enemies, mut velocities, mut grounded, mut players, _healths): Self::SystemData) {
        grounded.clear();

        // We need to collect collisions first to avoid double borrow issues if we were to try complex things,
        // but actually, if we iterate over entities and then query others, we might be okay if we don't borrow `positions` for the whole loop?
        // No, `join` borrows the storage.
        
        // 1. Platform Collision & Resolution
        let mut resolutions = Vec::new();
        for (entity, pos, vel, render, _player) in (&entities, &positions, &velocities, &renderables, &players).join() {
            let player_rect = (pos.x, pos.y, render.width, render.height);

            for (platform_pos, platform_render, _platform) in (&positions, &renderables, &platforms).join() {
                let platform_rect = (platform_pos.x, platform_pos.y, platform_render.width, platform_render.height);

                if check_aabb(player_rect, platform_rect) {
                    let mut new_pos = (pos.x, pos.y);
                    let mut new_vel = (vel.x, vel.y);
                    let mut on_ground = false;

                    // Vertical Resolution (Top/Bottom)
                    if vel.y > 0.0 && pos.y + render.height <= platform_pos.y + 15.0 {
                        new_pos.1 = platform_pos.y - render.height;
                        new_vel.1 = 0.0;
                        on_ground = true;
                    } else if vel.y < 0.0 && pos.y >= platform_pos.y + platform_render.height - 15.0 {
                        new_pos.1 = platform_pos.y + platform_render.height;
                        new_vel.1 = 0.0;
                    }
                    // Horizontal Resolution (Sides) - Simplified
                    else if vel.x > 0.0 && pos.x + render.width <= platform_pos.x + 10.0 {
                        new_pos.0 = platform_pos.x - render.width;
                        new_vel.0 = 0.0;
                    } else if vel.x < 0.0 && pos.x >= platform_pos.x + platform_render.width - 10.0 {
                        new_pos.0 = platform_pos.x + platform_render.width;
                        new_vel.0 = 0.0;
                    }
                    resolutions.push((entity, new_pos, new_vel, on_ground));
                }
            }
        }

        for (entity, new_pos, new_vel, on_ground) in resolutions {
            if let Some(pos) = positions.get_mut(entity) {
                pos.x = new_pos.0;
                pos.y = new_pos.1;
            }
            if let Some(vel) = velocities.get_mut(entity) {
                vel.x = new_vel.0;
                vel.y = new_vel.1;
            }
            if on_ground {
                grounded.insert(entity, Grounded).ok();
            }
        }

        // 2. Enemy Collisions (Damage)
        let mut damaged_players = Vec::new();
        for (player_entity, player_pos, player_render, _player) in (&entities, &positions, &renderables, &players).join() {
            let player_rect = (player_pos.x, player_pos.y, player_render.width, player_render.height);
            for (enemy_pos, enemy_render, _enemy) in (&positions, &renderables, &enemies).join() {
                let enemy_rect = (enemy_pos.x, enemy_pos.y, enemy_render.width, enemy_render.height);
                if check_aabb(player_rect, enemy_rect) {
                    damaged_players.push(player_entity);
                }
            }
        }

        // 3. Collectibles
        let mut to_remove = Vec::new();
        let mut score_updates = Vec::new();
        for (player_entity, player_pos, player_render, _player) in (&entities, &positions, &renderables, &players).join() {
            let player_rect = (player_pos.x, player_pos.y, player_render.width, player_render.height);
            for (collectible_entity, collectible_pos, collectible_render, _collectible) in (&entities, &positions, &renderables, &collectibles).join() {
                let collectible_rect = (collectible_pos.x, collectible_pos.y, collectible_render.width, collectible_render.height);
                if check_aabb(player_rect, collectible_rect) {
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
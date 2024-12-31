use specs::{System, ReadStorage, WriteStorage, Join, Entities};
use crate::components::{Position, Collidable, Velocity};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collidable>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, positions, collidables, mut velocities): Self::SystemData) {
        // Crée un vecteur pour stocker les modifications de vitesse
        let mut velocity_changes = Vec::new();

        // Première boucle pour détecter les collisions
        for (entity1, pos1, col1, vel1) in (&entities, &positions, &collidables, &velocities).join() {
            for (entity2, pos2, col2, vel2) in (&entities, &positions, &collidables, &velocities).join() {
                if entity1 == entity2 {
                    continue;
                }

                let dx = pos1.x - pos2.x;
                let dy = pos1.y - pos2.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < (col1.radius + col2.radius) {
                    // Collision détectée
                    // Stocke les modifications de vitesse dans le vecteur
                    velocity_changes.push((entity1, -vel1.x, -vel1.y));
                    velocity_changes.push((entity2, -vel2.x, -vel2.y));
                }
            }
        }

        // Applique les modifications de vitesse après la boucle
        for (entity, new_vx, new_vy) in velocity_changes {
            if let Some(vel) = velocities.get_mut(entity) {
                vel.x = new_vx;
                vel.y = new_vy;
            }
        }
    }
}
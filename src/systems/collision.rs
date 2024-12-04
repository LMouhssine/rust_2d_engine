use specs::{System, ReadStorage, Join};
use crate::components::{Position, Collidable};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collidable>,
    );

    fn run(&mut self, (positions, collidables): Self::SystemData) {
        for (pos1, col1, pos2, col2) in (&positions, &collidables, &positions, &collidables).join() {
            if std::ptr::eq(pos1, pos2) {
                continue;
            }
            
            let dx = pos1.x - pos2.x;
            let dy = pos1.y - pos2.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < (col1.radius + col2.radius) {
                // Collision détectée
                // Ici vous pourriez ajouter une logique de réponse aux collisions
            }
        }
    }
} 
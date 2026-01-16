use specs::{World, WorldExt, Builder};
use crate::components::*;

pub fn load_level(world: &mut World, level_data: &str) {
    let lines: Vec<&str> = level_data.lines().collect();
    
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos_x = x as f32 * 40.0;
            let pos_y = y as f32 * 40.0;
            
            match ch {
                '#' => { // Platform
                    world.create_entity()
                        .with(Position { x: pos_x, y: pos_y })
                        .with(Renderable { width: 40.0, height: 40.0, color: (100, 100, 100) })
                        .with(Platform)
                        .build();
                },
                'P' => { // Player
                    world.create_entity()
                        .with(Position { x: pos_x, y: pos_y })
                        .with(Velocity { x: 0.0, y: 0.0 })
                        .with(Renderable { width: 40.0, height: 40.0, color: (0, 255, 0) })
                        .with(Player { speed: 200.0, jump_force: 600.0, score: 0 })
                        .with(Health { current: 100, max: 100 })
                        .with(Gravity)
                        .build();
                },
                'E' => { // Enemy
                    world.create_entity()
                        .with(Position { x: pos_x, y: pos_y })
                        .with(Velocity { x: 100.0, y: 0.0 })
                        .with(Renderable { width: 40.0, height: 40.0, color: (255, 0, 0) })
                        .with(Enemy)
                        .with(Gravity)
                        .build();
                },
                'C' => { // Coin
                    world.create_entity()
                        .with(Position { x: pos_x, y: pos_y })
                        .with(Renderable { width: 20.0, height: 20.0, color: (255, 215, 0) })
                        .with(Collectible)
                        .build();
                },
                'G' => { // Goal
                    world.create_entity()
                        .with(Position { x: pos_x, y: pos_y })
                        .with(Renderable { width: 40.0, height: 60.0, color: (0, 255, 255) }) // Cyan Portal
                        .with(Goal)
                        .build();
                },
                '?' => { // Tutorial Hint (Square)
                    world.create_entity()
                        .with(Position { x: pos_x, y: pos_y })
                        .with(Renderable { width: 40.0, height: 40.0, color: (200, 200, 255) })
                        .build();
                },
                _ => {}
            }
        }
    }
}

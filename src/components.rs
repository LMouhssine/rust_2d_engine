use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub width: f32,
    pub height: f32,
    pub color: (u8, u8, u8), // Changed to u8 for SDL2 direct use
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub score: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collidable {
    #[allow(dead_code)]
    pub radius: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ParticleEmitter {
    pub rate: f32,
    pub lifetime: f32,
    pub color: (u8, u8, u8),
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Lifetime {
    pub remaining: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collectible;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Gravity;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Grounded;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Platform;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Enemy;

#[derive(Component, Debug)]
#[storage(VecStorage)]
#[allow(dead_code)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
#[allow(dead_code)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub timer: f32,
    pub frame_duration: f32,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Goal;

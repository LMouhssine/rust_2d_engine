use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub width: f32,
    pub height: f32,
    pub color: (f32, f32, f32),
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {
    pub speed: f32,
    pub score: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collidable {
    pub radius: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ParticleEmitter {
    pub rate: f32,
    pub lifetime: f32,
    pub color: (f32, f32, f32),
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
use specs::{Component, VecStorage};
use sdl2::pixels::Color;

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
pub struct Renderable {
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collidable {
    pub radius: f32,
}

pub struct DeltaTime(pub f32);
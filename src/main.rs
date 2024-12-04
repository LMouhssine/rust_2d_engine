// src/main.rs
mod components;
mod systems;
mod utils;

use specs::{World, WorldExt, Builder, DispatcherBuilder};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

use crate::components::*;
use crate::systems::*;

pub struct GameState {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Enregistrement des composants
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Renderable>();
        world.register::<Player>();
        world.register::<Collidable>();
        world.register::<ParticleEmitter>();

        // Création du dispatcher
        let dispatcher = DispatcherBuilder::new()
            .with(MovementSystem, "movement", &[])
            .with(CollisionSystem, "collision", &["movement"])
            .with(ParticleSystem, "particle", &["movement"])
            .build();

        // Création du joueur
        world.create_entity()
            .with(Position { x: 400.0, y: 300.0 })
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Renderable {
                width: 50.0,
                height: 50.0,
                color: Color::RGB(255, 255, 255),
            })
            .with(Player { speed: 300.0 })
            .with(Collidable { radius: 25.0 })
            .with(ParticleEmitter {
                rate: 10.0,
                lifetime: 1.0,
                color: Color::RGB(200, 200, 255),
            })
            .build();

        // Ajout d'obstacles
        for i in 0..5 {
            world.create_entity()
                .with(Position {
                    x: 100.0 + i as f32 * 150.0,
                    y: 200.0,
                })
                .with(Renderable {
                    width: 30.0,
                    height: 30.0,
                    color: Color::RGB(255, 100, 100),
                })
                .with(Collidable { radius: 15.0 })
                .build();
        }

        GameState { world, dispatcher }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.world.insert(DeltaTime(delta_time));
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window = video_subsystem.window("Moteur de Jeu 2D Avancé", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
        
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
        
    let mut event_pump = sdl_context.event_pump()?;
    let mut game_state = GameState::new();
    let mut last_update = Instant::now();
    
    let mut running = true;
    while running {
        // Calcul du delta time
        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Gestion des événements
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    handle_input(&mut game_state.world, key, true);
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    handle_input(&mut game_state.world, key, false);
                },
                _ => {}
            }
        }

        // Mise à jour
        game_state.update(delta_time);

        // Rendu
        canvas.set_draw_color(Color::RGB(0, 0, 40));
        canvas.clear();
        
        render_game(&game_state.world, &mut canvas)?;
        
        canvas.present();
    }

    Ok(())
}

// src/components.rs
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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ParticleEmitter {
    pub rate: f32,
    pub lifetime: f32,
    pub color: Color,
}

pub struct DeltaTime(pub f32);

// src/systems/mod.rs
mod movement;
mod collision;
mod particle;

pub use self::movement::MovementSystem;
pub use self::collision::CollisionSystem;
pub use self::particle::ParticleSystem;

// src/systems/movement.rs
use specs::{System, ReadStorage, WriteStorage, Join};
use crate::components::{Position, Velocity, DeltaTime};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        specs::Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut positions, velocities, delta_time): Self::SystemData) {
        for (pos, vel) in (&mut positions, &velocities).join() {
            pos.x += vel.x * delta_time.0;
            pos.y += vel.y * delta_time.0;
            
            // Garder dans les limites de l'écran
            pos.x = pos.x.clamp(0.0, 800.0);
            pos.y = pos.y.clamp(0.0, 600.0);
        }
    }
}
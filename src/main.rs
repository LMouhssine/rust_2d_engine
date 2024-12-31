mod components;
mod systems;
mod utils;

use specs::{World, WorldExt, Builder, DispatcherBuilder};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

use crate::components::{Player, Renderable, Position, Velocity, Collidable, ParticleEmitter, Collectible, Lifetime}; // Ajoute `Lifetime` ici
use crate::systems::{MovementSystem, CollisionSystem, ParticleSystem, CollectibleSystem};
use crate::utils::{handle_input, render_game};

pub struct GameState {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Renderable>();
        world.register::<Player>();
        world.register::<Collidable>();
        world.register::<ParticleEmitter>();
        world.register::<Lifetime>();
        world.register::<Collectible>();

        let dispatcher = DispatcherBuilder::new()
            .with(MovementSystem, "movement", &[])
            .with(CollisionSystem, "collision", &["movement"])
            .with(ParticleSystem, "particle", &["movement"])
            .with(CollectibleSystem, "collectible", &["collision"])
            .build();

        world.create_entity()
            .with(Position { x: 400.0, y: 300.0 })
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Renderable {
                width: 50.0,
                height: 50.0,
                color: (255.0, 255.0, 255.0),
            })
            .with(Player { speed: 300.0, score: 0 })
            .with(Collidable { radius: 25.0 })
            .with(ParticleEmitter {
                rate: 10.0,
                lifetime: 1.0,
                color: (200.0, 200.0, 255.0),
            })
            .build();

        for i in 0..5 {
            world.create_entity()
                .with(Position {
                    x: 100.0 + i as f32 * 150.0,
                    y: 200.0,
                })
                .with(Renderable {
                    width: 30.0,
                    height: 30.0,
                    color: (255.0, 100.0, 100.0),
                })
                .with(Collidable { radius: 15.0 })
                .with(Collectible)
                .build();
        }

        GameState { world, dispatcher }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.world.insert(delta_time);
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().map_err(|e| format!("Erreur d'initialisation SDL: {}", e))?;
    let video_subsystem = sdl_context.video().map_err(|e| format!("Erreur de sous-système vidéo: {}", e))?;
    
    let window = video_subsystem.window("Moteur de Jeu 2D Avancé", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| format!("Erreur de création de la fenêtre: {}", e))?;
        
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| format!("Erreur de création du canvas: {}", e))?;
        
    let mut event_pump = sdl_context.event_pump().map_err(|e| format!("Erreur de création de l'event pump: {}", e))?;
    let mut game_state = GameState::new();
    let mut last_update = Instant::now();
    
    let mut running = true;
    while running {
        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

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

        game_state.update(delta_time);
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 40));
        canvas.clear();
        
        render_game(&game_state.world, &mut canvas)?;
        canvas.present();
    }

    Ok(())
}
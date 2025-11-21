mod components;
mod systems;
mod utils;

use specs::{World, WorldExt, Builder, DispatcherBuilder};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

use crate::components::{Player, Renderable, Position, Velocity, Collidable, ParticleEmitter, Collectible, Lifetime, Gravity, Grounded, Platform};
use crate::systems::{MovementSystem, CollisionSystem, ParticleSystem, LogicSystem};
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
        world.register::<Gravity>();
        world.register::<Grounded>();
        world.register::<Platform>();

        let dispatcher = DispatcherBuilder::new()
            .with(MovementSystem, "movement", &[])
            .with(CollisionSystem, "collision", &["movement"])
            .with(ParticleSystem, "particle", &["movement"])
            .with(LogicSystem, "logic", &["movement"])
            .build();

        // Create Player
        world.create_entity()
            .with(Position { x: 100.0, y: 100.0 })
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Renderable {
                width: 40.0,
                height: 40.0,
                color: (0.0, 255.0, 0.0), // Green Player
            })
            .with(Player { speed: 200.0, jump_force: 500.0, score: 0 })
            .with(Collidable { radius: 20.0 })
            .with(Gravity)
            .with(ParticleEmitter {
                rate: 2.0, // Emit occasionally
                lifetime: 0.5,
                color: (100.0, 255.0, 100.0),
            })
            .build();

        // Create Ground Platform
        world.create_entity()
            .with(Position { x: 0.0, y: 550.0 })
            .with(Renderable {
                width: 800.0,
                height: 50.0,
                color: (100.0, 100.0, 100.0),
            })
            .with(Collidable { radius: 400.0 }) // Radius doesn't make sense for rect, but used in collision logic? 
            // My collision logic used radius for platform width/2? 
            // Let's check collision.rs: 
            // let platform_rect = (platform_pos.x, platform_pos.y, platform_collider.radius * 2.0, 20.0);
            // So radius * 2 = width. Width 800 -> Radius 400.
            .with(Platform)
            .build();

        // Create Floating Platforms
        let platforms = vec![
            (200.0, 400.0, 100.0),
            (400.0, 300.0, 100.0),
            (600.0, 200.0, 100.0),
        ];

        for (x, y, w) in platforms {
            world.create_entity()
                .with(Position { x, y })
                .with(Renderable {
                    width: w,
                    height: 20.0,
                    color: (150.0, 150.0, 255.0),
                })
                .with(Collidable { radius: w / 2.0 })
                .with(Platform)
                .build();
        }

        // Create Collectibles
        for i in 0..5 {
            world.create_entity()
                .with(Position {
                    x: 220.0 + i as f32 * 100.0,
                    y: 150.0 + (i % 2) as f32 * 100.0,
                })
                .with(Renderable {
                    width: 20.0,
                    height: 20.0,
                    color: (255.0, 215.0, 0.0), // Gold
                })
                .with(Collidable { radius: 10.0 })
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
    let sdl_context = sdl2::init().map_err(|e| format!("Init Error: {}", e))?;
    let video_subsystem = sdl_context.video().map_err(|e| format!("Video Error: {}", e))?;
    
    let window = video_subsystem.window("Rust 2D Platformer", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| format!("Window Error: {}", e))?;
        
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| format!("Canvas Error: {}", e))?;
        
    let mut event_pump = sdl_context.event_pump().map_err(|e| format!("Event Pump Error: {}", e))?;
    let mut game_state = GameState::new();
    let mut last_update = Instant::now();
    
    let mut running = true;
    while running {
        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;
        
        if delta_time > 0.0 {
            // println!("FPS: {:.2}", 1.0 / delta_time);
        }

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
        
        render_game(&game_state.world, &mut canvas)?;
        canvas.present();
    }

    Ok(())
}
mod components;
mod systems;
mod utils;

use specs::{World, WorldExt, DispatcherBuilder};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

use crate::components::{Player, Renderable, Position, Velocity, Collidable, ParticleEmitter, Collectible, Lifetime, Gravity, Grounded, Platform, Enemy, Health};
use crate::systems::{MovementSystem, CollisionSystem, ParticleSystem, LogicSystem, EnemyAISystem};
use crate::utils::{handle_input, render_game, level_loader::load_level};

#[derive(PartialEq, Clone, Copy)]
pub enum GameMode {
    Menu,
    Playing,
    GameOver,
}

pub struct GameState {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    pub mode: GameMode,
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
        world.register::<Enemy>();
        world.register::<Health>();

        let dispatcher = DispatcherBuilder::new()
            .with(EnemyAISystem, "enemy_ai", &[])
            .with(MovementSystem, "movement", &["enemy_ai"])
            .with(CollisionSystem, "collision", &["movement"])
            .with(ParticleSystem, "particle", &["movement"])
            .with(LogicSystem, "logic", &["movement"])
            .build();

        // Load Initial Level
        let level_data = "
####################
#                  #
#   C   C   C      #
#  ### ### ###     #
#                  #
#      E           #
#    #####         #
#            C     #
#   P      #####   #
####################
";
        load_level(&mut world, level_data);

        GameState { world, dispatcher, mode: GameMode::Menu }
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
                    if game_state.mode == GameMode::Menu {
                        game_state.mode = GameMode::Playing;
                    } else if game_state.mode == GameMode::Playing {
                        handle_input(&mut game_state.world, key, true);
                    } else if game_state.mode == GameMode::GameOver {
                        game_state.mode = GameMode::Menu;
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if game_state.mode == GameMode::Playing {
                        handle_input(&mut game_state.world, key, false);
                    }
                },
                _ => {}
            }
        }

        if game_state.mode == GameMode::Playing {
            game_state.update(delta_time);
            
            // Check Lose Condition (Fall off screen)
            let positions = game_state.world.read_storage::<Position>();
            let players = game_state.world.read_storage::<Player>();
            use specs::Join;
            for (_player, pos) in (&players, &positions).join() {
                if pos.y > 600.0 {
                    game_state.mode = GameMode::GameOver;
                }
            }
            
            // Win condition (Score >= 100)
            for player in (&players).join() {
                if player.score >= 100 {
                    // Could also be a win state, but for now just loop or game over
                    game_state.mode = GameMode::GameOver;
                }
            }
        }
        
        render_game(&game_state.world, &mut canvas, game_state.mode)?;
        canvas.present();
    }

    Ok(())
}
mod components;
mod systems;
mod utils;

use specs::{World, WorldExt, DispatcherBuilder};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

use crate::components::{Player, Renderable, Position, Velocity, Collidable, ParticleEmitter, Collectible, Lifetime, Gravity, Grounded, Platform, Enemy, Health, Goal};
use crate::systems::{MovementSystem, CollisionSystem, ParticleSystem, LogicSystem, EnemyAISystem};
use crate::utils::{handle_input, render_game, level_loader::load_level};

#[derive(PartialEq, Clone, Copy)]
pub enum GameMode {
    Menu,
    Tutorial,
    Playing,
    GameOver,
    Win,
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
        world.register::<Goal>();

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

fn check_aabb_simple(r1: (f32, f32, f32, f32), r2: (f32, f32, f32, f32)) -> bool {
    r1.0 < r2.0 + r2.2 &&
    r1.1 < r2.1 + r2.3 &&
    r1.0 + r1.2 > r2.0 &&
    r1.1 + r1.3 > r2.1
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
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match game_state.mode {
                        GameMode::Menu => {
                            game_state.mode = GameMode::Tutorial;
                            // Tutorial Level
                            game_state.world.delete_all();
                            let tutorial_map = "
####################
#       ?          #
#       ?          #
#    P  ?   C     G#
#   #####  ###   ###
####################";
                            load_level(&mut game_state.world, tutorial_map);
                        },
                        GameMode::Tutorial | GameMode::Playing => {
                            handle_input(&mut game_state.world, keycode, true);
                        },
                        GameMode::GameOver | GameMode::Win => {
                            game_state.mode = GameMode::Menu;
                            game_state.world.delete_all();
                        }
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if game_state.mode == GameMode::Tutorial || game_state.mode == GameMode::Playing {
                        handle_input(&mut game_state.world, keycode, false);
                    }
                },
                _ => {}
            }
        }

        if game_state.mode == GameMode::Menu {
            // Wait for key press to start tutorial
        } else if game_state.mode == GameMode::Tutorial {
            game_state.update(delta_time);
            
            // Goal check in Tutorial triggers Playing
            let mut transition_to_playing = false;
            {
                let positions = game_state.world.read_storage::<Position>();
                let players = game_state.world.read_storage::<Player>();
                let goals = game_state.world.read_storage::<Goal>();
                let renderables = game_state.world.read_storage::<Renderable>();
                use specs::Join;
                for (_player, pos, render) in (&players, &positions, &renderables).join() {
                    let player_rect = (pos.x, pos.y, render.width, render.height);
                    for (goal_pos, goal_render, _goal) in (&positions, &renderables, &goals).join() {
                        let goal_rect = (goal_pos.x, goal_pos.y, goal_render.width, goal_render.height);
                        if check_aabb_simple(player_rect, goal_rect) {
                            transition_to_playing = true;
                            break;
                        }
                    }
                    if transition_to_playing { break; }
                }
            }

            if transition_to_playing {
                // Switch to Playing!
                game_state.mode = GameMode::Playing;
                // Reload for the main level
                game_state.world.delete_all();
                let level_data = "
####################
#                  #
#                  #
#       C          #
#      ###         #
#             E    #
#    P      #####  #
#   ###            #
#                 G#
####################";
                load_level(&mut game_state.world, level_data);
            }
        } else if game_state.mode == GameMode::Playing {
            game_state.update(delta_time);
            
            {
                let positions = game_state.world.read_storage::<Position>();
                let players = game_state.world.read_storage::<Player>();
                let goals = game_state.world.read_storage::<Goal>();
                let renderables = game_state.world.read_storage::<Renderable>();
                use specs::Join;
                
                // Check Lose Condition (Fall off screen)
                for (_player, pos) in (&players, &positions).join() {
                    if pos.y > 600.0 {
                        game_state.mode = GameMode::GameOver;
                    }
                }
                
                // Check Win Condition (Reach Goal)
                if game_state.mode == GameMode::Playing {
                    for (_player, pos, render) in (&players, &positions, &renderables).join() {
                        let player_rect = (pos.x, pos.y, render.width, render.height);
                        for (goal_pos, goal_render, _goal) in (&positions, &renderables, &goals).join() {
                            let goal_rect = (goal_pos.x, goal_pos.y, goal_render.width, goal_render.height);
                            if check_aabb_simple(player_rect, goal_rect) {
                                game_state.mode = GameMode::Win;
                                break;
                            }
                        }
                        if game_state.mode == GameMode::Win { break; }
                    }
                }
            }
        }
        
        render_game(&game_state.world, &mut canvas, game_state.mode)?;
        canvas.present();
    }

    Ok(())
}
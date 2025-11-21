use sdl2::rect::Rect;
use specs::World;
use specs::WorldExt;
use crate::components::{Position, Renderable, Velocity, Player, Grounded};

pub fn render_game(world: &World, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    use specs::Join;

    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
    let players = world.read_storage::<Player>();

    // Check Win Condition
    let mut win = false;
    for player in players.join() {
        if player.score >= 50 {
            win = true;
        }
    }

    if win {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(50, 150, 50)); // Win Green
    } else {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 20, 40)); // Normal Blue
    }
    canvas.clear();
    
    // Render Platforms/Players/Collectibles
    for (pos, render) in (&positions, &renderables).join() {
        let color = render.color;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0 as u8, color.1 as u8, color.2 as u8));
        canvas.fill_rect(Rect::new(
            pos.x as i32,
            pos.y as i32,
            render.width as u32,
            render.height as u32,
        ))?;
    }

    // Render HUD (Score)
    let players = world.read_storage::<Player>();
    for player in players.join() {
        for i in 0..player.score {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 215, 0));
            canvas.fill_rect(Rect::new(10 + (i as i32 * 15), 10, 10, 10))?;
        }
    }

    Ok(())
}

pub fn handle_input(world: &mut World, keycode: sdl2::keyboard::Keycode, pressed: bool) {
    use specs::Join;
    let mut velocities = world.write_storage::<Velocity>();
    let players = world.read_storage::<Player>();
    let grounded = world.read_storage::<Grounded>();
    let entities = world.entities();
    
    // We need to know if player is grounded to jump
    // But we can't borrow grounded immutably and velocities mutably easily if we iterate?
    // Actually we can.
    
    for (entity, vel, player) in (&entities, &mut velocities, &players).join() {
        let is_grounded = grounded.get(entity).is_some();
        let _speed = if pressed { player.speed } else { 0.0 };
        
        match keycode {
            sdl2::keyboard::Keycode::Left => {
                if pressed { vel.x = -player.speed; } 
                else if vel.x < 0.0 { vel.x = 0.0; } // Stop only if we were moving left
            },
            sdl2::keyboard::Keycode::Right => {
                if pressed { vel.x = player.speed; }
                else if vel.x > 0.0 { vel.x = 0.0; }
            },
            sdl2::keyboard::Keycode::Space => {
                if pressed && is_grounded {
                    vel.y = -player.jump_force;
                }
            },
            _ => {}
        }
    }
}
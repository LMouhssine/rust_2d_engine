pub mod level_loader;
use sdl2::rect::Rect;
use specs::World;
use specs::WorldExt;
use crate::components::{Position, Renderable, Velocity, Player, Grounded};
use crate::GameMode;

pub fn render_game(world: &World, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, mode: GameMode) -> Result<(), String> {
    use specs::Join;

    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
    let players = world.read_storage::<Player>();

    if mode == GameMode::Menu {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(10, 10, 20));
        canvas.clear();
        // TODO: Render Menu Text
        return Ok(());
    }

    if mode == GameMode::GameOver {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(50, 0, 0));
        canvas.clear();
        // TODO: Render Game Over Text
        return Ok(());
    }

    // Normal Gameplay background
    canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 20, 40)); 
    canvas.clear();
    
    // Render Platforms/Players/Collectibles
    for (pos, render) in (&positions, &renderables).join() {
        let color = render.color;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
        canvas.fill_rect(Rect::new(
            pos.x as i32,
            pos.y as i32,
            render.width as u32,
            render.height as u32,
        ))?;
    }

    // Render HUD (Score Bar)
    let players = world.read_storage::<Player>();
    let positions = world.read_storage::<Position>();
    let entities = world.entities();

    for (player, pos, _entity) in (&players, &positions, &entities).join() {
        // Draw score bar
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 215, 0));
        canvas.fill_rect(Rect::new(10, 10, (player.score as u32 * 2).min(200), 10))?;

        // Lose condition: Fall off screen
        if pos.y > 600.0 {
            // We need a way to signal GameOver to main.rs or update a resource
            // For now, let's just use a simple println and we'll handle it in main.rs loop if possible
            // Actually, we should probably add a GameState resource.
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
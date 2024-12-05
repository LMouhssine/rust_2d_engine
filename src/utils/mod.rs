use sdl2::rect::Rect;
use specs::World;
use specs::WorldExt;
use crate::components::{Position, Renderable, Velocity, Player};

pub fn render_game(world: &World, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    use specs::Join;
    
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
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

    Ok(())
}

pub fn handle_input(world: &mut World, keycode: sdl2::keyboard::Keycode, pressed: bool) {
    use specs::Join;
    let mut velocities = world.write_storage::<Velocity>();
    let players = world.read_storage::<Player>();

    for (vel, player) in (&mut velocities, &players).join() {
        let speed = if pressed { player.speed } else { 0.0 };
        match keycode {
            sdl2::keyboard::Keycode::Up => vel.y = -speed,
            sdl2::keyboard::Keycode::Down => vel.y = speed,
            sdl2::keyboard::Keycode::Left => vel.x = -speed,
            sdl2::keyboard::Keycode::Right => vel.x = speed,
            _ => {}
        }
    }
}
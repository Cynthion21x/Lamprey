pub mod content;
pub mod rendering;
mod config;
use rendering::window;
use content::sprite;
mod game;
use sdl2::{keyboard::Keycode, pixels::Color, event::Event};

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;  

    let mut window_m = window::WindowM::new(&video_subsystem);
    
    let texture_maker = window_m.canvas.texture_creator();
    let sprites = sprite::SpriteMan::new(&texture_maker);
    
    let mut core = game::Game::new(event_pump);
    
    window_m.canvas.set_draw_color(Color::RGB(0, 255, 0));
    window_m.canvas.clear();
    window_m.canvas.present();
    
    while core.running {
       core.input();
       core.update();
       core.render();
    }
    
    Ok(())

}
